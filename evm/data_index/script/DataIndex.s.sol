// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";

import {DataIndex} from "../src/DataIndex.sol";

contract DataIndexScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        new DataIndex();

        console2.log("deployer addr:", address(this));

        vm.stopBroadcast();
    }
}
