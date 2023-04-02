#!/bin/bash
set -e
. $SH_DIR/base/init-vars.sh

AGENT_BALANCE=10000
FAUCET=cw-croncat-faucet

Signer=$($BINARY keys show signer --address)
echo "Signer: $Signer"

if [[ -z "$Signer" ]]; then
  echo "${Red}Signer is not set. Signer must be set before address initialization ${NoColor}"
  exit 1
fi
echo "Initializing addresses..."
#Agent
junod keys delete $FAUCET -y
FAUCET_SEED_PHRASE="very priority voice drink cloud advance wait pave dose useful erode proud just absorb east eyebrow unaware prize old brand above arrow east aim"
$BINARY keys show $FAUCET 2>/dev/null || echo $FAUCET_SEED_PHRASE | $BINARY keys add $FAUCET --recover
FAUCET_ADDRESS=$($BINARY keys show $FAUCET --address)
sleep 5

#Faucet
junod keys delete agent -y
AGENT_SEED="olive soup parade family educate congress hurt dwarf mom this position hungry unaware aunt swamp sunny analyst wrestle fashion main knife start coffee air"
echo $AGENT_SEED | $BINARY keys add agent --recover
AGENT_ADDR=$($BINARY keys show agent --address)
sleep 5

echo "${Cyan}Agent :" $AGENT_ADDR "${NoColor}"
echo "${Cyan}Signer :" $Signer "${NoColor}"

echo "${Yellow}Sending funds to users...${NoColor}"
$BINARY tx bank send signer $FAUCET_ADDRESS "$OWNER_BALANCE"ujunox $NODE --chain-id $CHAIN_ID --yes --broadcast-mode block --sign-mode direct --fees=6000$STAKE
$BINARY tx bank send $FAUCET $AGENT_ADDR "$AGENT_BALANCE"ujunox $NODE --chain-id $CHAIN_ID --yes --broadcast-mode block --sign-mode direct --fees=6000$STAKE
echo "${Cyan}Funds sent...${NoColor}"
