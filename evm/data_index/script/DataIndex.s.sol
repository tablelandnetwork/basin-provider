// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";

import {DataIndexTwo} from "../src/DataIndexTwo.sol";

contract DataIndexScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        DataIndexTwo dataIndex = new DataIndexTwo();

        // Create a dummy deal
        dataIndex.createDeal(
            DataIndexTwo.Deal({
                id: 1,
                expiration: block.timestamp + 3600,
                status: "active",
                minerId: "f01234",
                selectorPath: "path/to/selector",
                pieceCid: "Qm456",
                updatedTs: 0
            }),
            "abdc1234",
            address(this)
        );

        vm.stopBroadcast();
    }
}
