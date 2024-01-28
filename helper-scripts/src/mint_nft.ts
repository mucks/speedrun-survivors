import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Metaplex, keypairIdentity, toMetaplexFile, toBigNumber, bundlrStorage } from "@metaplex-foundation/js";
import * as fs from 'fs';
import secret from '../secret.json';
import * as bs58 from 'bs58';

const SOLANA_RPC = 'https://api.devnet.solana.com';
const SOLANA_CONNECTION = new Connection(SOLANA_RPC);

const secretKey = bs58.decode(secret);

const WALLET = Keypair.fromSecretKey(secretKey);


const METAPLEX = Metaplex.make(SOLANA_CONNECTION)
    .use(keypairIdentity(WALLET))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: SOLANA_RPC,
        timeout: 60000,
    }));

const CONFIG = {
    uploadPath: 'uploads/',
    imgFileName: 'image.png',
    imgType: 'image/png',
    imgName: 'QuickNode Pixel',
    description: 'Pixel infrastructure for everyone!',
    attributes: [
        { trait_type: 'Speed', value: 'Quick' },
        { trait_type: 'Type', value: 'Pixelated' },
        { trait_type: 'Background', value: 'QuickNode Blue' }
    ],
    sellerFeeBasisPoints: 500,//500 bp = 5%
    symbol: 'QNPIX',
    creators: [
        { address: WALLET.publicKey, share: 100 }
    ]
};


async function uploadImage(filePath: string, fileName: string): Promise<string> {

    const imgBuffer = fs.readFileSync(filePath + fileName);

    const imgMetaplexFile = toMetaplexFile(imgBuffer, fileName);

    const imgUri = await METAPLEX.storage().upload(imgMetaplexFile);
    console.log(`   Image URI:`, imgUri);
    return imgUri;

}

async function uploadMetadata(imgUri: string, imgType: string, nftName: string, description: string, attributes: { trait_type: string, value: string }[]) {
    console.log(`Step 2 - Uploading Metadata`);

    const { uri } = await METAPLEX
        .nfts()
        .uploadMetadata({
            name: nftName,
            description: description,
            image: imgUri,
            attributes: attributes,
            properties: {
                files: [
                    {
                        type: imgType,
                        uri: imgUri,
                    },
                ]
            }
        }).run();
    console.log('   Metadata URI:', uri);
    return uri;

}

async function mintNft(metadataUri: string, name: string, sellerFee: number, symbol: string, creators: { address: PublicKey, share: number }[]) {
    console.log(`Step 3 - Minting NFT`);
    const { nft } = await METAPLEX
        .nfts()
        .create({
            uri: metadataUri,
            name: name,
            sellerFeeBasisPoints: sellerFee,
            symbol: symbol,
            creators: creators,
            isMutable: false,
        }).run();
    console.log(`   Success!🎉`);
    console.log(`   Minted NFT: https://explorer.solana.com/address/${nft.address}?cluster=devnet`);
}

async function main() {
    console.log(`Minting ${CONFIG.imgName} to an NFT in Wallet ${WALLET.publicKey.toBase58()}.`);

    const imgUri = await uploadImage(CONFIG.uploadPath, CONFIG.imgFileName);
    const metadataUri = await uploadMetadata(imgUri, CONFIG.imgType, CONFIG.imgName, CONFIG.description, CONFIG.attributes);

    mintNft(metadataUri, CONFIG.imgName, CONFIG.sellerFeeBasisPoints, CONFIG.symbol, CONFIG.creators);


}

main();