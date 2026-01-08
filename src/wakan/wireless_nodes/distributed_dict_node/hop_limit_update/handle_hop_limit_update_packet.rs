#![allow(dead_code)]
use crate::wakan::{
    wireless_nodes::distributed_dict_node::{
        HopLimitUpdate, HLU_SMOOTHING_DENOMINATOR, HLU_SMOOTHING_NUMERATOR,
    },
    Time,
};

type NeedToBroadcastHLU = bool;

impl HopLimitUpdate {
    fn handle_hop_limit_update_packet(
        &mut self,
        requested_hop_limit: u8,
        now: Time,
    ) -> NeedToBroadcastHLU {
        if requested_hop_limit < self.hop_limit {
            return false;
        }

        // todo!("replave with with Welford algo, https://www.johndcook.com/blog/standard_deviation/ {:?}", {
        // update HLU rate satistics
        let current_interval = now - self.time_of_last_hlu_received;

        let diff = current_interval.abs_diff(self.hlu_mean_interval);

        let n1 = (HLU_SMOOTHING_DENOMINATOR - HLU_SMOOTHING_NUMERATOR) * self.hlu_variance as u64;
        let n2 = HLU_SMOOTHING_NUMERATOR * diff;
        let n3 = (n1 * HLU_SMOOTHING_DENOMINATOR) + (n2 * HLU_SMOOTHING_DENOMINATOR);
        if let Ok(x) = (n3 / (HLU_SMOOTHING_DENOMINATOR * HLU_SMOOTHING_DENOMINATOR)).try_into() {
            self.hlu_variance = x;
        }

        if requested_hop_limit > self.hop_limit {
            // update hop_limit
            self.hop_limit = requested_hop_limit;
            return true;
        } else if now - self.time_of_last_hlu_broadcast > self.hlu_mean_interval {
            return true;
        }
        return false;
    }
}
