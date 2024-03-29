# Aderyn Analysis Report

This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities.
# Table of Contents

- [Summary](#summary)
  - [Files Summary](#files-summary)
  - [Files Details](#files-details)
  - [Issue Summary](#issue-summary)
- [NC Issues](#nc-issues)
  - [NC-1: Title for WeirdErc20NotHandledDetector](#nc-1-title-for-weirderc20nothandleddetector)
  - [NC-2: Title for WeirdErc721NotHandledDetector](#nc-2-title-for-weirderc721nothandleddetector)


# Summary

## Files Summary

| Key | Value |
| --- | --- |
| .sol Files | 1 |
| Total nSLOC | 10 |


## Files Details

| Filepath | nSLOC |
| --- | --- |
| src/Counter.sol | 10 |
| **Total** | **10** |


## Issue Summary

| Category | No. of Issues |
| --- | --- |
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 0 |
| NC | 2 |


# NC Issues

## NC-1: Title for WeirdErc20NotHandledDetector

Description for WeirdErc20NotHandledDetector

- Found in src/Counter.sol [Line: 8](foundry_workspace/src/Counter.sol#L8)

	```solidity
	        number = newNumber;
	```



## NC-2: Title for WeirdErc721NotHandledDetector

Description for WeirdErc721NotHandledDetector

- Found in src/Counter.sol [Line: 5](foundry_workspace/src/Counter.sol#L5)

	```solidity
	    uint256 public number;
	```

- Found in src/Counter.sol [Line: 7](foundry_workspace/src/Counter.sol#L7)

	```solidity
	    function setNumber(uint256 newNumber) public {
	```

- Found in src/Counter.sol [Line: 8](foundry_workspace/src/Counter.sol#L8)

	```solidity
	        number = newNumber;
	```



