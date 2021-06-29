use super::command::run;
use anyhow::Error;
use serde;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_json;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLDPNeighbor {
    #[serde(rename = "chassis_type")]
    remote_chassis_type: String,
    #[serde(rename = "chassis_id")]
    remote_chassis: String,
    #[serde(rename = "l_port_id")]
    local_port: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    hold_time: u8,
    system_capability: String,
    enabled_capability: String,
    port_type: String,
    #[serde(rename = "port_id")]
    remote_port: String,
    #[serde(rename = "mgmt_addr_type")]
    management_address_type: String,
    #[serde(rename = "mgmt_addr")]
    management_address: String,
    #[serde(rename = "mgmt_addr_ipv6_type")]
    management_address_ipv6_type: String,
    #[serde(rename = "mgmt_addr_ipv6")]
    management_address_ipv6: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborsTable {
    #[serde(rename = "ROW_nbor")]
    neighbors: Vec<LLDPNeighbor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowLLDPNeighborsResult {
    #[serde(rename = "TABLE_nbor")]
    neighbors_table: NeighborsTable,
}

/// Return the complete list of LLDP neighbors
pub fn get_lldp_neighbors() -> Result<Vec<LLDPNeighbor>, Error> {
    let neighbors_json = run("show lldp neighbors | json".to_string())?;
    let neighbors: ShowLLDPNeighborsResult = serde_json::from_str(neighbors_json.stdout.as_str())?;
    Ok(neighbors.neighbors_table.neighbors)
}

#[cfg(test)]
mod tests {
    use crate::xdp::{LLDPNeighbor, ShowLLDPNeighborsResult};
    use anyhow::Error;
    use std::fs;

    #[test]
    fn good_neighbor_format() -> Result<(), Error> {
        let data = fs::read_to_string("tests/data/lldp_neighbors_ok.json")?;
        let result: ShowLLDPNeighborsResult = serde_json::from_str(data.as_str())?;
        let neighbors = result.neighbors_table.neighbors;

        let test_neighbor = neighbors.first().unwrap();
        let my_neighbor = LLDPNeighbor {
            remote_chassis_type: "Locally Assigned".to_string(),
            remote_chassis: "test-hostname".to_string(),
            local_port: "mgmt0".to_string(),
            hold_time: 120,
            system_capability: "BR".to_string(),
            enabled_capability: "BR".to_string(),
            port_type: "Interface Name".to_string(),
            remote_port: "Gi1/21".to_string(),
            management_address_type: "IPV4".to_string(),
            management_address: "1.2.3.4".to_string(),
            management_address_ipv6_type: "Address not advertised".to_string(),
            management_address_ipv6: "not advertised".to_string(),
        };

        assert_eq!(
            my_neighbor.remote_chassis_type,
            test_neighbor.remote_chassis_type
        );
        assert_eq!(my_neighbor.remote_chassis, test_neighbor.remote_chassis);
        assert_eq!(my_neighbor.local_port, test_neighbor.local_port);
        assert_eq!(my_neighbor.hold_time, test_neighbor.hold_time);
        assert_eq!(
            my_neighbor.system_capability,
            test_neighbor.system_capability
        );
        assert_eq!(
            my_neighbor.enabled_capability,
            test_neighbor.enabled_capability
        );
        assert_eq!(my_neighbor.port_type, test_neighbor.port_type);
        assert_eq!(my_neighbor.remote_port, test_neighbor.remote_port);
        assert_eq!(
            my_neighbor.management_address_type,
            test_neighbor.management_address_type
        );
        assert_eq!(
            my_neighbor.management_address,
            test_neighbor.management_address
        );
        assert_eq!(
            my_neighbor.management_address_ipv6_type,
            test_neighbor.management_address_ipv6_type
        );
        assert_eq!(
            my_neighbor.management_address_ipv6,
            test_neighbor.management_address_ipv6
        );

        Ok(())
    }
}
