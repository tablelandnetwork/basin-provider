// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console2} from "forge-std/Test.sol";
import {DataIndex} from "../src/DataIndex.sol";

contract DataIndexTest is Test {
    DataIndex public dataIndex;

    constructor() {
        dataIndex = new DataIndex();
    }

    // Test the CreateDealInfo function
    function testCreateDealInfo() public {
        // Define the input parameters
        string memory publicationId = "123456";
        uint64 dealId = 1;
        string memory selectorPath = "path/to/selector";

        // Call the CreateDealInfo function
        dataIndex.createDealInfo(
            DataIndex.DealInfo({
                id: dealId,
                selectorPath: selectorPath,
                publicationId: publicationId
            }),
            address(this)
        );

        // Get the deal for the publication
        DataIndex.DealInfo[] memory deals = dataIndex.dealsByOwner(
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
        DataIndex.DealInfo memory deal1 = DataIndex.DealInfo({
            id: 1,
            selectorPath: "path1",
            publicationId: "publication123"
        });
        DataIndex.DealInfo memory deal2 = DataIndex.DealInfo({
            id: 2,
            selectorPath: "path2",
            publicationId: "publication123"
        });
        DataIndex.DealInfo memory deal3 = DataIndex.DealInfo({
            id: 3,
            selectorPath: "path3",
            publicationId: "publication321"
        });

        // Add the test deals to the contract
        dataIndex.createDealInfo(deal1, owner);
        dataIndex.createDealInfo(deal2, owner);
        dataIndex.createDealInfo(deal3, owner);

        // Call the function being tested
        DataIndex.DealInfo[] memory deals = dataIndex
            .dealsByOwnerForPublication(owner, publicationId);

        // Check that the correct deals were returned
        assertEq(deals.length, 2, "Expected 2 deals to be returned");
        assertEq(
            deals[0].id,
            deal1.id,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[0].publicationId,
            deal1.publicationId,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[0].selectorPath,
            deal1.selectorPath,
            "Returned deal does not match expected deal"
        );

        assertEq(
            deals[1].id,
            deal2.id,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[1].publicationId,
            deal2.publicationId,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[1].selectorPath,
            deal2.selectorPath,
            "Returned deal does not match expected deal"
        );

        deals = dataIndex.dealsByOwnerForPublication(owner, "publication321");

        // Check that the correct deals were returned
        assertEq(deals.length, 1, "Expected one deal to be returned");
        assertEq(
            deals[0].id,
            deal3.id,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[0].publicationId,
            deal3.publicationId,
            "Returned deal does not match expected deal"
        );
        assertEq(
            deals[0].selectorPath,
            deal3.selectorPath,
            "Returned deal does not match expected deal"
        );
    }
}
