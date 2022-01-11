use crate::internal::*;

pub struct CommitKey<'a> {
    pub ptr: *mut IrminCommitKey,

    #[allow(dead_code)]
    pub(crate) repo: UntypedRepo<'a>,
}

impl<'a> Drop for CommitKey<'a> {
    fn drop(&mut self) {
        unsafe { irmin_commit_key_free(self.ptr) }
    }
}

impl<'a> CommitKey<'a> {
    pub fn to_string(&self) -> Result<IrminString, Error> {
        let t = unsafe { irmin_type_commit_key(self.repo.ptr) };
        let s = unsafe { irmin_value_to_string(t, self.ptr as *mut _) };
        unsafe { irmin_type_free(t) }
        IrminString::wrap(s)
    }
}
