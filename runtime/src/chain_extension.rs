use codec::Encode;
use frame_support::log::{
    error,
    trace,
};
use pallet_contracts::chain_extension::{
    ChainExtension,
    Environment,
    Ext,
    InitState,
    RetVal,
    SysConfig,
    UncheckedFrom,
};
use sp_runtime::DispatchError;
use crate::Runtime;
use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Randomness;

/// Contract extension for `FetchRandom`
#[derive(Default)]
pub struct FetchRandomExtension;

impl ChainExtension<Runtime> for FetchRandomExtension {
    fn call<E: Ext>(
        &mut self,
        mut env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId:
            UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let func_id = env.func_id();
        match func_id {
            1101 => {
                let mut env = env.buf_in_buf_out();
                let arg: [u8; 32] = env.read_as()?;
                let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
                let random_slice = random_seed.encode();
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&random_slice, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
            }
            8801 => {
                let ext = env.ext();

                // 获取缓冲区的指针
                let mut env = env.buf_in_buf_out();
                // 将缓冲区的数据转换成具体的对象
                let arg: [u8; 32] = env.read_as()?;
                let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
                let random_slice = random_seed.encode();
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&random_slice, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
            }

            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}