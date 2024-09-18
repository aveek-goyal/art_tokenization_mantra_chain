import { useCallback, useState } from 'react';
import { useAccount, useCosmWasmClient } from "graz";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { CONTRACT_ADDRESS } from '../chain';
import { GasPrice } from "@cosmjs/stargate";
import { coins } from "@cosmjs/proto-signing";

export function useNftContract() {
  const { data: account } = useAccount();
  const { data: cosmWasmClient } = useCosmWasmClient();
  const [loading, setLoading] = useState(false);

  const getSigningClient = useCallback(async () => {
    if (!window.keplr) throw new Error("Keplr not found");
    await window.keplr.enable("mantra-hongbai-1");
    const offlineSigner = window.keplr.getOfflineSigner("mantra-hongbai-1");
    const gasPrice = GasPrice.fromString('0.025uaum');
    return await SigningCosmWasmClient.connectWithSigner("https://rpc.hongbai.mantrachain.io", offlineSigner, { gasPrice });
  }, []);

  const instantiateContract = useCallback(async (initMsg) => {
    if (!account) return;
    setLoading(true);
    try {
      const signingClient = await getSigningClient();
      const result = await signingClient.instantiate(
        account.bech32Address,
        initMsg.code_id,
        initMsg,
        "Instantiate NFT Contract",
        "auto"
      );
      setLoading(false);
      return result;
    } catch (error) {
      console.error("Failed to instantiate contract:", error);
      setLoading(false);
      throw error;
    }
  }, [account, getSigningClient]);

  const queryConfig = useCallback(async () => {
    if (!cosmWasmClient) return null;
    setLoading(true);
    try {
      const nftDetails = await cosmWasmClient.queryContractSmart(CONTRACT_ADDRESS, { nft_details: {} });
      let metadata = {};
      if (nftDetails.token_uri) {
        const response = await fetch(nftDetails.token_uri);
        metadata = await response.json();
      }
      const nft = {
        name: nftDetails.name,
        description: metadata.description || nftDetails.symbol,
        image: metadata.image ? `https://gateway.pinata.cloud/ipfs/${metadata.image.slice(7)}` : null,
        mint_price: Number(nftDetails.mint_price.amount) / 1000000,
        max_mint: nftDetails.max_mints,
        token_count: nftDetails.token_count
      };
      setLoading(false);
      return nft;
    } catch (error) {
      console.error("Failed to fetch NFT details:", error);
      setLoading(false);
      throw error;
    }
  }, [cosmWasmClient]);

  const mintNft = useCallback(async () => {
    if (!account) return;
    setLoading(true);
    try {
      const signingClient = await getSigningClient();
      const nftDetails = await queryConfig();
      if (!nftDetails) {
        throw new Error("Failed to fetch NFT details");
      }
      const mintPrice = nftDetails.mint_price;
      await signingClient.execute(
        account.bech32Address,
        CONTRACT_ADDRESS,
        { mint: {} },  // Updated to match the new contract structure
        "auto",
        "",
        coins(mintPrice * 1000000, "uom")
      );
      setLoading(false);
    } catch (error) {
      console.error("Failed to mint NFT:", error);
      setLoading(false);
      throw error;
    }
  }, [account, getSigningClient, queryConfig]);

  return { instantiateContract, queryConfig, mintNft, loading, setLoading };
}