// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// Path: contracts/RepositoryContract.sol
contract RepositoryContract {
    struct Object {
        string hash;

        bytes ipfs_url;

        address pusher;
    }

    struct Ref {
        bytes data;
        bool is_active;

        address pusher;
    }

    mapping(string => Object) public objects;
    mapping(string => Ref) public refs;

    Object[] public objectsById;
    Ref[] public refsById;

    bytes public config;

    event ObjectSaved(string hash, bytes ipfs_url, address pusher);
    event RefAdded(string ref, bytes ipfs_url, address pusher);
    event ConfigUpdated(bytes config);

    function saveObject(string memory _hash, bytes memory _ipfs_url) public {
        if (objects[_hash].ipfs_url.length > 0) {
            return;
        }

        Object memory object = Object(_hash, _ipfs_url, msg.sender);

        objects[_hash] = object;
        objectsById.push(object);

        emit ObjectSaved(_hash, _ipfs_url, msg.sender);
    }

    function addRef(string memory _ref, bytes memory _data) public {
        if (refs[_ref].data.length > 0) {
            return;
        }

        address pusher = msg.sender;

        refs[_ref] = Ref(_data, true, pusher);
        refsById.push(Ref(_data, true, pusher));

        emit RefAdded(_ref, _data, pusher);
    }

    function updateConfig(bytes memory _config) public {
        config = _config;

        emit ConfigUpdated(_config);
    }

    function getConfig() public view returns (bytes memory) {
        return config;
    }

    function getObjectById(uint256 _id) public view returns (Object memory) {
        return objectsById[_id];
    }

    function getObject(string memory _hash) public view returns (Object memory) {
        return objects[_hash];
    }

    function isObjectExist(string memory _hash) public view returns (bool) {
        return objects[_hash].ipfs_url.length > 0;
    }

    function checkObjects(string[] memory _hashes) public view returns (bool[] memory) {
        bool[] memory results = new bool[](_hashes.length);

        for (uint256 i = 0; i < _hashes.length; i++) {
            results[i] = objects[_hashes[i]].ipfs_url.length > 0;
        }

        return results;
    }

    function addObjects(string[] memory _hashes, bytes[] memory _ipfs_urls) public {
        for (uint256 i = 0; i < _hashes.length; i++) {
            if (objects[_hashes[i]].ipfs_url.length > 0) {
                continue;
            }

            Object memory object = Object(_hashes[i], _ipfs_urls[i], msg.sender);

            objects[_hashes[i]] = object;
            objectsById.push(object);

            emit ObjectSaved(_hashes[i], _ipfs_urls[i], msg.sender);
        }
    }

    function addRefs(string[] memory _refs, bytes[] memory _data) public {
        for (uint256 i = 0; i < _refs.length; i++) {
            if (refs[_refs[i]].data.length > 0) {
                continue;
            }

            address pusher = msg.sender;
            refs[_refs[i]] = Ref(_data[i], true, pusher);

            emit RefAdded(_refs[i], _data[i], pusher);
        }
    }

    function getRefs() public view returns (Ref[] memory) {
        return refsById;
    }

    function getObjects() public view returns (Object[] memory) {
        return objectsById;
    }

    function getObjectsLength() public view returns (uint256) {
        return objectsById.length;
    }

    function getRefsLength() public view returns (uint256) {
        return refsById.length;
    }
}