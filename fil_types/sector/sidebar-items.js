initSidebarItems({"constant":[["MAX_SECTOR_NUMBER","The maximum assignable sector number. Raising this would require modifying our AMT implementation."]],"enum":[["RegisteredAggregateProof","Seal proof type which defines the version and sector size."],["RegisteredPoStProof","Proof of spacetime type, indicating version and sector size of the proof."],["RegisteredSealProof","Seal proof type which defines the version and sector size."],["SectorSize","SectorSize indicates one of a set of possible sizes in the network."]],"mod":[["post",""]],"struct":[["AggregateSealVerifyInfo","Information needed to verify an aggregated seal proof."],["AggregateSealVerifyProofAndInfos",""],["SealVerifyInfo","Information needed to verify a seal proof."],["SealVerifyParams","SealVerifyParams is the structure of information that must be sent with a message to commit a sector. Most of this information is not needed in the state tree but will be verified in sm.CommitSector. See SealCommitment for data stored on the state tree for each sector."],["SectorID","Sector ID which contains the sector number and the actor ID for the miner."]],"type":[["InteractiveSealRandomness","Randomness used when verifying a seal proof. This is just a seed value."],["SealRandomness","Randomness used for Seal proofs."],["SectorNumber","SectorNumber is a numeric identifier for a sector. It is usually relative to a miner."],["SectorQuality","Unit of sector quality"],["Spacetime","The unit of spacetime committed to the network"],["StoragePower","Unit of storage power (measured in bytes)"]]});