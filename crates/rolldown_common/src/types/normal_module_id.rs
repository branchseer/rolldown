use bincode::{Decode, Encode};

index_vec::define_index_type! {
    #[derive(Encode, Decode)]
    pub struct NormalModuleId = u32;
}
