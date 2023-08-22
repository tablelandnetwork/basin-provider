// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console2} from "forge-std/Test.sol";
import {DataIndexTwo} from "../src/DataIndexTwo.sol";

contract DataIndexTest is Test {
    DataIndexTwo public dataIndex;

    event DealCreated(
        uint256 dealId,
        string indexed publicationId,
        address indexed ownerId
    );

    constructor() {
        dataIndex = new DataIndexTwo();
    }

    // Test the createDeal function
    function testCreateDeal() public {
        // Define the input parameters
        string memory publicationId = "123456";
        uint256 dealId = 1;
        uint256 dealExpiration = block.timestamp + 3600;
        string memory dealStatus = "active";
        string memory miner = "f01234";
        string memory selectorPath = "path/to/selector";
        string memory pieceCid = "Qm456";

        DataIndexTwo.Deal memory deal = DataIndexTwo.Deal({
            id: dealId,
            expiration: dealExpiration,
            status: dealStatus,
            minerId: miner,
            selectorPath: selectorPath,
            pieceCid: pieceCid,
            updatedTs: 0
        });

        // Assert that events were emitted correctly
        vm.expectEmit(address(dataIndex));
        emit DealCreated(dealId, publicationId, address(this));

        // Call the createDeal function
        dataIndex.createDeal(deal, publicationId, address(this));

        // Get the deal for the publication
        deal = dataIndex.getDeal(dealId);

        // Assert that the deal was created correctly
        assertEq(
            deal.expiration,
            dealExpiration,
            "The deal expiration should be correct"
        );
        assertEq(deal.status, dealStatus, "The deal status should be correct");
        assertEq(deal.minerId, miner, "The miner address should be correct");
        assertEq(
            deal.selectorPath,
            selectorPath,
            "The selector path should be correct"
        );
        assertEq(deal.pieceCid, pieceCid, "The piece CID should be correct");
        assertEq(
            deal.updatedTs,
            block.timestamp,
            "The updated timestamp should be correct"
        );
    }
}
