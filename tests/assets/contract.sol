// SPDX-License-Identifier: MIT

pragma solidity >=0.8.2;

contract TheMostValuableFile {

    string cid;

    constructor(string memory _cid) {
        cid = _cid;
    }

    function get_cid() public view returns (string memory) {
        return cid;
    }
}