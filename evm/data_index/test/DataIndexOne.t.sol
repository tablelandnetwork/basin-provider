// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {DataIndexOne} from "../src/DataIndexOne.sol";

contract DataIndexTest is Test {
    DataIndexOne public dataIndex;

    constructor() {
        dataIndex = new DataIndexOne();
    }

    // Test the createPublication function
    function testCreatePublication() public {
        // Define the input parameters
        address owner = address(0x123);
        string memory publicationId = "123456";

        // Call the createPublication function
        dataIndex.createPublication(owner, publicationId);

        // Get the publication for the owner
        DataIndexOne.Publication[] memory publications = dataIndex
            .getPublicationsByOwner(owner);

        // Assert that the publication was created correctly
        assertEq(
            publications.length,
            1,
            "There should be one publication for the owner"
        );
        assertEq(
            publications[0].owner,
            owner,
            "The owner of the publication should be correct"
        );
        assertEq(
            publications[0].id,
            publicationId,
            "The ID of the publication should be correct"
        );
    }

    // Test the CreateDealInfo function
    function testCreateDealInfo() public {
        // Define the input parameters
        string memory publicationId = "123456";
        uint256 dealId = 1;
        uint256 dealExpiration = block.timestamp + 3600;
        string memory dealStatus = "ACTIVE";
        string memory miner = "f01278";
        string memory selectorPath = "path/to/selector";

        // Call the CreateDealInfo function
        dataIndex.CreateDealInfo(
            DataIndexOne.Deal({
                id: dealId,
                expiration: dealExpiration,
                status: dealStatus,
                minerId: miner,
                selectorPath: selectorPath
            }),
            publicationId,
            address(this)
        );

        // Get the deal for the publication
        DataIndexOne.Deal[] memory deals = dataIndex.getDealsByPublicationId(
            publicationId
        );

        // Assert that the deal was created correctly
        assertEq(
            deals[0].expiration,
            dealExpiration,
            "The deal expiration should be correct"
        );
        assertEq(
            deals[0].status,
            dealStatus,
            "The deal status should be correct"
        );
        assertEq(
            deals[0].minerId,
            miner,
            "The miner address should be correct"
        );
        assertEq(
            deals[0].selectorPath,
            selectorPath,
            "The selector path should be correct"
        );
    }
}
