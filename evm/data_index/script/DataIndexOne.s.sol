// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";

import {DataIndexOne} from "../src/DataIndexOne.sol";

contract DataIndexScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        DataIndexOne dataIndex = new DataIndexOne();

        // TODO: create a dummy deal info for testing, remove later
        dataIndex.createDealInfo(
            DataIndexOne.DealInfo({
                id: 132565,
                selectorPath: "path/to/selector",
                publicationId: "abcd123456"
            }),
            address(this)
        );

        console2.log("deployer addr:", address(this));

        vm.stopBroadcast();
    }
}
