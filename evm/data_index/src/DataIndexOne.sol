// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Ownable} from "openzeppelin/access/Ownable.sol";

contract DataIndexOne is Ownable {
    struct Deal {
        uint256 id;
        uint256 expiration;
        string status;
        string minerId;
        string selectorPath;
    }

    // Event to log when a deal is added or updated
    event DealCreated(
        uint256 dealId,
        string indexed publicationId,
        address indexed ownerId
    );

    struct Publication {
        address owner;
        string id;
    }

    /// @dev A mapping that stores the publications for each owner.
    mapping(address => Publication[]) public ownerPublications;

    /// @dev A mapping that stores the data deals for each key.
    mapping(string => Deal[]) public publicationDeals;

    /// @dev Sets the data deal for a given data owner.
    ///      Can only be called by the contract owner.
    /// @param owner The owner address to set the deal for.
    /// @param publicationId The deal ID.
    function createPublication(
        address owner,
        string calldata publicationId
    ) public onlyOwner {
        ownerPublications[owner].push(
            Publication({owner: owner, id: publicationId})
        );
    }

    /// @dev Sets the data deal for a given publication.
    ///      Can only be called by the contract owner.
    /// @param deal The Filecoin deal object.
    /// @param publicationId The publication ID to set the deal for.
    /// @param owner owner of the data.
    function createDealInfo(
        Deal calldata deal,
        string calldata publicationId,
        address owner
    ) public onlyOwner {
        publicationDeals[publicationId].push(
            Deal({
                id: deal.id,
                expiration: deal.expiration,
                status: deal.status,
                minerId: deal.minerId,
                selectorPath: deal.selectorPath
            })
        );

        emit DealCreated(deal.id, publicationId, owner);
    }

    /// @dev Returns the publications for a given data owner.
    /// @param owner The owner address to get the deals for.
    /// @return publications The deals for the given data owner.
    function publicationsByOwner(
        address owner
    ) public view returns (Publication[] memory publications) {
        return ownerPublications[owner];
    }

    /// @dev Returns the deals for a given publication.
    /// @param publicationId The publication ID to get the deals for.
    /// @return deals The deals for the given publication.
    function dealsByPublicationId(
        string calldata publicationId
    ) public view returns (Deal[] memory deals) {
        return publicationDeals[publicationId];
    }
}
