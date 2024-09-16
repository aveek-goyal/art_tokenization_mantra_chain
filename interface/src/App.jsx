import React, { useState, useEffect, useCallback } from "react";
import {
  Box,
  VStack,
  HStack,
  Heading,
  Text,
  Input,
  Button,
  Flex,
  useColorMode,
  Container,
  Image,
  useToast,
  Center,
  Spacer,
  Badge,
} from "@chakra-ui/react";
import { MoonIcon, SunIcon } from "@chakra-ui/icons";
import { useAccount, useConnect, useDisconnect } from "graz";
import { useNftContract } from './hooks/useNFTContract';
import { checkKeplrInstalled, getKeplrInstallUrl } from './utils/keplrUtils';

export default function App() {
  const { data: account, isConnected, isConnecting, isReconnecting } = useAccount();
  const { connect } = useConnect();
  const { disconnect } = useDisconnect();
  const { queryConfig, mintNft, loading, setLoading } = useNftContract();
  const { colorMode, toggleColorMode } = useColorMode();
  const toast = useToast();

  const [config, setConfig] = useState(null);

  const connectWallet = async () => {
    if (!checkKeplrInstalled()) {
      const installUrl = getKeplrInstallUrl();
      if (window.confirm("Keplr wallet is not installed. Would you like to install it now?")) {
        window.open(installUrl, '_blank');
      }
    } else {
      try {
        connect({ chainId: "mantra-hongbai-1" });
      } catch (error) {
        console.error("Failed to connect:", error);
        showToast("Failed to connect. Please make sure Keplr is set up correctly.", "error");
      }
    }
  };

  useEffect(() => {
    if (isConnected) {
      queryConfig().then(setConfig).catch(error => {
        console.error("Failed to fetch config:", error);
        showToast("Error fetching config. Please try again later.", "error");
      });
    }
  }, [isConnected, queryConfig, toast]);

  const handleMint = useCallback(() => {
    mintNft().then(() => {
      showToast("NFT minted successfully!", "success");
    }).catch(error => {
      console.error("Failed to mint NFT:", error);
      setLoading(false);
      showToast("Error minting NFT. Please try again.", "error");
    });
  }, [mintNft, setLoading]);

  const showToast = (message, status) => {
    toast({
      title: status === "error" ? "Error" : "Success",
      description: message,
      status: status,
      duration: 3000,
      isClosable: true,
    });
  };

  return (
    <Box minH="100vh" minW="100vw" bg={colorMode === "dark" ? "gray.800" : "gray.100"}>
      <Container maxW="container.xl" py={8}>
        <Flex justify="space-between" align="center" mb={8}>
          <Heading size="xl">NFT Gallery</Heading>
          <HStack spacing={4}>
            {account && (
              <Text fontSize="sm">
                {account.bech32Address.slice(0, 8)}...{account.bech32Address.slice(-4)}
              </Text>
            )}
            <Button
              onClick={() => isConnected ? disconnect() : connectWallet()}
              isLoading={isConnecting || isReconnecting}
              loadingText="Connecting"
            >
              {isConnected ? "Disconnect" : "Connect Wallet"}
            </Button>
            <Button onClick={toggleColorMode}>
              {colorMode === "light" ? <MoonIcon /> : <SunIcon />}
            </Button>
          </HStack>
        </Flex>

        {isConnected && config ? (
          <VStack spacing={8} align="stretch">
            <Center>
              <Image
                src={config.image}
                alt={config.name}
                maxH="500px"
                objectFit="contain"
                borderRadius="lg"
                boxShadow="xl"
              />
            </Center>
            <VStack spacing={4} align="center">
              <Heading size="lg" colorScheme="black">{config.name}</Heading>
              <Text fontSize="md" textAlign="center" maxW="600px" colorScheme="black">
                {config.description}
              </Text>
              <HStack>
                <Badge colorScheme="blue">Max Mints: {config.max_mint}</Badge>
                <Badge colorScheme="green">Mint Price: {config.mint_price} OM</Badge>
              </HStack>
                <Button
                  onClick={handleMint}
                  isLoading={loading}
                  loadingText="Minting"
                  colorScheme="blue"
                  size="lg"
                >
                  Mint NFT
                </Button>
            </VStack>
          </VStack>
        ) : (
          <Center h="50vh">
            <VStack spacing={6}>
              <Heading size="lg">Welcome to NFT Gallery</Heading>
              <Text>Connect your wallet to view and mint NFTs</Text>
              <Button size="lg" onClick={connectWallet} colorScheme="blue">
                Connect Wallet
              </Button>
            </VStack>
          </Center>
        )}
      </Container>
    </Box>
  );
}