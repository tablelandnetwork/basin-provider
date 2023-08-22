// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console2} from "forge-std/Test.sol";
import {DataIndexOne} from "../src/DataIndexOne.sol";

contract DataIndexTest is Test {
    DataIndexOne public dataIndex;

    constructor() {
        dataIndex = new DataIndexOne();
    }

    // Test the CreateDealInfo function
    function testCreateDealInfo() public {
        // Define the input parameters
        string memory publicationId = "123456";
        uint64 dealId = 1;
        string memory selectorPath = "path/to/selector";

        // Call the CreateDealInfo function
        dataIndex.createDealInfo(
            DataIndexOne.DealInfo({
                id: dealId,
                selectorPath: selectorPath,
                publicationId: publicationId
            }),
            address(this)
        );

        // Get the deal for the publication
        DataIndexOne.DealInfo[] memory deals = dataIndex.dealsByOwner(
            address(this)
        );

        assertEq(deals.length, 1, "Number of deals should be 1");
        assertEq(deals[0].id, dealId, "Publication ID should be correct");
        assertEq(
            deals[0].publicationId,
            publicationId,
            "Publication ID should be correct"
        );
        assertEq(
            deals[0].selectorPath,
            selectorPath,
            "The selector path should be correct"
        );
    }

    function testDealsByOwnerForPublication() public {
        // Set up the contract and accounts
        address owner = address(0x123);
        string memory publicationId = "publication123";

        // Create some test deals
        DataIndexOne.DealInfo memory deal1 = DataIndexOne.DealInfo({
            id: 1,
            selectorPath: "path1",
            publicationId: "publication123"
        });
        DataIndexOne.DealInfo memory deal2 = DataIndexOne.DealInfo({
            id: 2,
            selectorPath: "path2",
            publicationId: "publication123"
        });
        DataIndexOne.DealInfo memory deal3 = DataIndexOne.DealInfo({
            id: 3,
            selectorPath: "path3",
            publicationId: "publication321"
        });

        // Add the test deals to the contract
        dataIndex.createDealInfo(deal1, owner);
        dataIndex.createDealInfo(deal2, owner);
        dataIndex.createDealInfo(deal3, owner);

        // Call the function being tested
        DataIndexOne.DealInfo[] memory deals = dataIndex.dealsByOwnerForPublication(
            owner,
            publicationId
        );

        // Check that the correct deals were returned
        assertEq(deals.length, 2, "Expected 2 deals to be returned");

        deals = dataIndex.dealsByOwnerForPublication(owner, "publication321");

        // Check that the correct deals were returned
        assertEq(deals.length, 1, "Expected one deal to be returned");
        assertEq(
            deals[0].id,
            deal3.id,
            "Returned deal does not match expected deal"
        );
    }
}
