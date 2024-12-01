#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    interface AOC2024 {
      function solve11(string calldata input) external view returns (uint32 result);
      function solve12(string calldata input) external view returns (uint32 result);
      // function solve01(string calldata input) external view returns (uint32 result);
      // function solve02(string calldata input) external view returns (uint32 result);
  }
);
