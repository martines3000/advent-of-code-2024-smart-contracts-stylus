#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    interface Solution {
      function solvepart1(string calldata input) external view returns (uint32 result);
      function solvepart2(string calldata input) external view returns (uint32 result);
  }
);
