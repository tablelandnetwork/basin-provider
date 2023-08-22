// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import "openzeppelin/access/Ownable.sol";

contract DataIndexTwo is Ownable {
    struct Deal {
        uint256 id;
        uint256 expiration;
        string status;
        string minerId;
        string selectorPath;
        string pieceCid;
        uint256 updatedTs;
    }

    // Mapping from deal ID to Filecoin deal details
    mapping(uint256 => Deal) public deals;

    // Event to log when a deal is added or updated
    event DealCreated(
        uint256 dealId,
        string indexed publicationId,
        address indexed ownerId
    );

    /// Event to log when a deal's status is updated
    event DealStatusUpdated(
        uint256 dealId,
        string status,
        string indexed publicationId,
        address indexed ownerId
    );

    // Custom error for non-existent deal
    error DealDoesNotExist(uint256 dealId);

    // Function to add a deal
    function createDeal(
        Deal calldata deal,
        string calldata publicationId,
        address owner
    ) public {
        deals[deal.id] = Deal({
            id: deal.id,
            expiration: deal.expiration,
            status: deal.status,
            minerId: deal.minerId,
            selectorPath: deal.selectorPath,
            pieceCid: deal.pieceCid,
            updatedTs: block.timestamp
        });

        emit DealCreated(deal.id, publicationId, owner);
    }

    // Function to update the status of an existing deal
    /* function updateDealStatus(
        uint256 dealId,
        string calldata newStatus,
        string calldata publicationId,
        address owner
    ) public {
        if (bytes(deals[dealId].status).length == 0) {
            revert DealDoesNotExist(dealId);
        }
        deals[dealId].status = newStatus;
        deals[dealId].updatedTs = block.timestamp;

        emit DealStatusUpdated(dealId, newStatus, publicationId, owner);
    }
 */
    // Function to retrieve a deal's details using its deal ID
    function getDeal(uint256 _dealId) public view returns (Deal memory) {
        return deals[_dealId];
    }
}
