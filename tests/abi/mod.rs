#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    contract AOC2024 {
        // function number() external view returns (uint256 number);
        // function setNumber(uint256 new_number) external;
        function solve01(string input) external returns (uint32 result);
        function solve02(string input) external returns (uint32 result);
    }
);
