#![allow(dead_code)]
use crate::wakan::{DistributedDictNode, Time, wireless_nodes::distributed_dict_node::{HLU_SMOOTHING_DENOMINATOR, HLU_SMOOTHING_NUMERATOR}};

impl DistributedDictNode {
    fn handle_hop_limit_update_packet(&mut self, requested_hop_limit: u8, now: Time){
        if requested_hop_limit < self.hop_limit {
            return;
        }

        // update HLU rate satistics
        let current_interval = now - self.last_hlu_received_time;

        let diff = current_interval.abs_diff(self.hlu_mean_interval);

        let n1 = (HLU_SMOOTHING_DENOMINATOR -  HLU_SMOOTHING_NUMERATOR) * self.hlu_variance as u64;
        let n2 = HLU_SMOOTHING_NUMERATOR * diff;
        let n3 = (n1*HLU_SMOOTHING_DENOMINATOR)+(n2*HLU_SMOOTHING_DENOMINATOR); 
        if let Ok(x) = (n3 / (HLU_SMOOTHING_DENOMINATOR * HLU_SMOOTHING_DENOMINATOR)).try_into(){
            self.hlu_variance = x;
        }

        if requested_hop_limit > self.hop_limit {
            // update hop_limit
            self.hop_limit = requested_hop_limit;
            self.need_to_broadcast_hlu = true;
        } else if now - self.last_hlu_broadcast_time > self.hlu_mean_interval {
            self.need_to_broadcast_hlu = true;
        }

    }
}