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
  }, [account, getSigningClient]);

  const queryConfig = useCallback(async () => {
    if (!cosmWasmClient) return null;
    setLoading(true);
    const nftdetails = await cosmWasmClient.queryContractSmart(CONTRACT_ADDRESS, { nft_details: {} });
    const data = nftdetails.token_uri;
    const response = await fetch(data);
    const metadata = await response.json();
    const nft = {name: metadata.name, description:metadata.description, image: `https://gateway.pinata.cloud/ipfs/${metadata.image.slice(7)}`, mint_price: ((nftdetails.mint_price.amount)/1000000), max_mint: nftdetails.max_mints}
    setLoading(false);
    return nft;
  }, [cosmWasmClient]);

  const mintNft = useCallback(async () => {
    if (!account) return;
    setLoading(true);
    const signingClient = await getSigningClient();
    const nftDetails = await queryConfig();
    if (!nftDetails) {
      setLoading(false);
      throw new Error("Failed to fetch NFT details");
    }
    const mintPrice = nftDetails.mint_price;
    await signingClient.execute(
      account.bech32Address,
      CONTRACT_ADDRESS,
      { mint: { owner: account.bech32Address, extension: {} } },
      "auto",
      "",
      coins(mintPrice * 1000000, "uom")  
    );
    setLoading(false);
  }, [account, getSigningClient, queryConfig]);

  return { instantiateContract, queryConfig, mintNft, loading, setLoading };
}