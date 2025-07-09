//! Data types and functions to interact with permissions in CHERI.

/// The complete set of architectural permissions.
// Taken from cheriot-rtos' `cheri.h`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Permission {
    /// Capability refers to global memory (this capability may be stored anywhere).
    Global = 0,

    /// Global capabilities can be loaded through this capability.  Without this permission, any
    /// capability loaded via this capability will have `Global` and `LoadGlobal` removed.
    LoadGlobal = 1,

    /// Capability may be used to store.  Any store via a capability without this permission will
    /// trap.
    Store = 2,

    /// Capabilities with store permission may be loaded through this capability.  Without this,
    /// any loaded capability will have `LoadMutable` and `Store` removed.
    LoadMutable = 3,

    /// This capability may be used to store capabilities that do not have `Global` permission.
    StoreLocal = 4,

    /// This capability can be used to load.
    Load = 5,

    /// Any load and store permissions on this capability convey the right to load or store
    /// capabilities in addition to data.
    LoadStoreCapability = 6,

    /// If installed as the program counter capability, running code may access privileged system
    /// registers.
    AccessSystemRegisters = 7,

    /// This capability may be used as a jump target and used to execute instructions.
    Execute = 8,

    /// This capability may be used to unseal other capabilities.  The 'address' range is in the
    /// sealing type namespace and not in the memory namespace.
    Unseal = 9,

    /// This capability may be used to seal other capabilities.  The 'address' range is in the
    /// sealing type namespace and not in the memory namespace.
    Seal = 10,

    /// Software defined permission bit, no architectural meaning.
    User0 = 11,
}

impl Permission {
    pub const fn bit(&self) -> u32 {
        1 << (*self as u32)
    }
}

pub struct PermissionSet {
    raw_permissions: u32,
}

impl PermissionSet {
    pub const fn empty() -> Self {
        Self { raw_permissions: 0 }
    }

    const fn as_raw(&self) -> u32 {
        self.raw_permissions
    }

    pub fn from_iter<I: Iterator<Item = Permission>>(iter: I) -> Self {
        let mut ret = Self::empty();
        let iter = iter.into_iter();

        for i in iter {
            ret.add_permission(i);
        }

        ret
    }
    pub fn contains(&self, permission: Permission) -> bool {
        permission.bit() & self.raw_permissions == permission.bit()
    }

    pub const fn add_permission(&mut self, permission: Permission) {
        self.raw_permissions |= permission.bit();
    }
}

impl core::fmt::Display for PermissionSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        if self.contains(Permission::Global) {
            write!(f, "G")?;
        }

        if self.contains(Permission::LoadGlobal) {
            write!(f, "<Lg>")?;
        }

        if self.contains(Permission::Store) {
            write!(f, "W")?;
        }

        if self.contains(Permission::LoadMutable) {
            write!(f, "m")?;
        }

        if self.contains(Permission::StoreLocal) {
            write!(f, "<Sl>")?;
        }

        if self.contains(Permission::Load) {
            write!(f, "R")?;
        }

        if self.contains(Permission::LoadStoreCapability) {
            write!(f, "c")?;
        }

        if self.contains(Permission::AccessSystemRegisters) {
            write!(f, "s")?;
        }

        if self.contains(Permission::Execute) {
            write!(f, "X")?;
        }

        if self.contains(Permission::Unseal) {
            write!(f, "u")?;
        }

        if self.contains(Permission::Seal) {
            write!(f, "S")?;
        }

        write!(f, ")")
    }
}

#[macro_export]
macro_rules! perms {
    {$($p: expr),*} => {
        const {
            let mut ret = PermissionSet::empty();
            $(
                ret.add_permission($p);
            )*
            ret
        }
    };
}

pub use perms;

unsafe extern "cheri-libcall" {
    #[link_name = "_Z13check_pointerPKvjjb"]
    pub fn _check_pointer(
        ptr: *const core::ffi::c_void,
        space: usize,
        raw_permissions: u32,
        check_stack_needed: bool,
    ) -> bool;
}

pub trait __InternalIntoRaw {
    fn into_raw(&self) -> *const Self;
}

impl<T> __InternalIntoRaw for *const T {
    fn into_raw(&self) -> *const Self {
        self
    }
}

impl<T> __InternalIntoRaw for *mut T {
    fn into_raw(&self) -> *const Self {
        self
    }
}

impl<T> __InternalIntoRaw for &T {
    fn into_raw(&self) -> *const Self {
        self
    }
}

pub fn check_pointer<T: __InternalIntoRaw>(
    ptr: T,
    space: usize,
    permissions: &PermissionSet,
    check_stack_needed: bool,
) -> bool {
    let should_check_stack = check_stack_needed && permissions.contains(Permission::Global);
    unsafe {
        _check_pointer(
            ptr.into_raw() as *const _,
            space,
            permissions.as_raw(),
            should_check_stack,
        )
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn libcall_tour() {
    let ret = check_pointer::<*const ()>(
        core::ptr::null(),
        0,
        &perms! {Permission::Load, Permission::Execute},
        false,
    );
    crate::println!("Checking null ptr gives: {ret}");

    static X: u32 = 0;

    let mut perms = perms! {Permission::Load};
    let ret = check_pointer(&X, 0, &perms, false);
    crate::println!("Checking ptr to static with perms {perms} gives: {ret}",);

    perms.add_permission(Permission::Execute);
    let ret = check_pointer(&X, 0, &perms, false);
    crate::println!("Checking ptr to static with perms {perms} gives: {ret}",);
}
