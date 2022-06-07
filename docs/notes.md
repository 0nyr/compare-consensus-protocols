# Notes

### Teacher ideas

DAG - tamgle (IOTA) -> dag de block, problème de rool back..

HashGraph (Hedera)

### Blockchain types

* Linear - Merkel chain blockchains like Bitcoin
* Non-linear, like DAGs, or parallel chains (MASSA)

# Presentation

### What is consensus

merriam-webster :

1 - accord général : UNANIMITÉ

2 - Le jugement auquel sont parvenus la plupart des intéressés

3 - solidarité de groupe dans le sentiment et la croyance

Larousse:

Accord et consentement du plus grand nombre, de l'opinion publique

Procédure qui consiste à dégager un accord sans procéder à un vote formel, ce qui évite de faire apparaître les objections et les abstentions.

Wikipedia:

Un problème fondamental dans l'informatique distribuée et les systèmes multi-agents est d'obtenir une fiabilité globale du système en présence d'un certain nombre de processus défectueux.

Ex : cloud computing, la synchronisation d'horloge, le PageRank, la formation d'opinion, les réseaux électriques intelligents, le contrôle de drones, l'équilibrage de charge, la blockchain...

### Byzantine Generals

Importance de la coordination.

Soit un système de n composants, dont t sont malhonnêtes, et en supposant uniquement un canal point à point entre tous les composants.

Chaque fois qu'un composant A essaie de diffuser une valeur x, les autres composants sont autorisés à discuter entre eux et à vérifier la cohérence de la diffusion de A, et finalement à se fixer sur une valeur commune y.

Propriété : Le système est dit résistant aux fautes byzantines si un composant A peut diffuser une valeur x, alors :

Si A est honnête, alors toutes les composantes honnêtes s'accordent sur la valeur x.
Dans tous les cas, tous les composants honnêtes s'accordent sur la même valeur y.

Formulation du problème sous son nom actuel par Lamport en 1982: "The Byzantine Generals Problem". A la base nommé par rapport aux généraux albanais, le nom fut changé pour une civilisation disparue...

## Creating Consensus

### General Solutions: pBFT

si n est le nombre de généraux au total, et t est le nombre de traîtres dans ce n, alors il n'y a de solutions au problème que lorsque n > 3t et que la communication est synchrone

PRACTICAL BYZANTINE FAULT TOLERANCE

Machine a états répliquée,

* nœuds  classés séquentiellement (1 leader, des suiveurs)
* nœuds communiquent: l'objectif: nœuds honnêtes parviennent à un accord sur l'état du système
* communication: 2 fonctions: origine des messages + non modifié
* Noeud malveillants !=> 1/3 de tous les noeuds
* PoW: + de noeud = + safe

Cycles de consensus pBFT (opinions) :
demande du client au nœud leader
le chef diffuse la demande
les nœuds répondent au client.
le client attend f+1 réponses similaires

### PoW

### PoSpace: Burstcoin, SpaceMint

or **Proof-Of-Capacity**

PoSpace consomme de l'espace disque et incite les mineurs avec l'espace disque le plus considérable alloué à un bloc. Implémentée à l'aide des graphes, cette structure de données est utilisée pour résoudre le jeu du cailloux. Le jeu de pebbling consiste à store le hash des sommets dans un graphe uniquement si tous les sommets parents ont été eux même stockés.

Pebble fait référence au stockage des valeurs de hachage des parents, et la suppression du caillou fait référence à la libération de la mémoire.

Toutes les solutions réalisables au problème sont générées aléatoirement, appelées parcelles (plots). Ces tracés sont stockés sur les disques et résolus à l'aide d'un algorithme appelé algorithme de Shabal. Une fois les solutions calculées, les mineurs comparent leurs solutions, et la solution avec la meilleure complexité temporelle et spatiale est récompensée par le bloc suivant.

### PoET

par Intel Corporation. L'algorithme est principalement utilisé dans les registres de blockchain permissioned. Le matériel utilisé dans PoET est spécialement conçu pour ce protocole. Par exemple, Intel Software Guarded Extension (SGX) est utilisé dans les réseaux utilisant PoET.

Pas d'anonymat

Chaque nœud du réseau se voit attribuer un temps d'attente aléatoire. Le premier nœud à terminer la période choisie au hasard valide le nouveau bloc. Le matériel spécialisé met le processeur en veille pendant le temps d'attente, ce qui se répète sur tous les blocs du réseau.

### PoS

demande aux utilisateurs de mettre une certaine quantité de leurs pièces « en jeu » pour avoir une chance d'être sélectionnés pour valider un bloc de transactions et recevoir cette récompense de bloc. Les acteurs malveillants risquent de **perdre** leur mise.


Avant de parler de la méthode en elle même, remontons à ses origines. Le géant des puces électroniques, Intel, a développé en 2016 Software Guard Extension (SGX), un outil permettant de créer des “enclaves” chiffrées. Traduction : SGX va créer des zones sécurisées et distinctes, où vos données seront stockées, ne pourront être modifiables, et seront chiffrées. Concrètement, SGX va faire 2 choses :

* Un composant va attester que le **code** compilé dans l’enclave a été réalisé correctement dans un  **environnement de confiance** . Cette **attestation** sera une preuve que le code est **authentique** pour des acteurs extérieurs.
* L’enclave informatique dans laquelle se trouve le code **ne peut être modifiée** par des applications tierces, et **n’interagit pas** avec les autres espaces de stockage. Cet **isolement** permet entre autres de conserver  **l’intégrité du code** .

En effet, lorsque vous souhaitez participer à une blockchain utilisant le POET, vous allez télécharger le code et faire parvenir **l’attestation de SGX** pour certifier que le code que vous allez utiliser est bien l’original, utilisé par les autres membres du réseau. Ensuite, cela fonctionne comme une **loterie** : chaque nœud se voit attribuer une  **durée définie aléatoirement par l’algorithme** . Le nœud qui a son compte à rebours finissant le premier va en envoyer la preuve au reste du réseau, et pourra alors valider le bloc.

**Nothing at stake**: « rien en jeu ». En cas de fork, qu'il s'agisse d'un fork accidentel ou d'une tentative malveillante de réécrire l'historique et d'annuler une transaction, la stratégie optimale pour tout mineur est de miner sur chaque chaîne, afin que le mineur obtienne sa récompense, quel que soit le fork qui gagne. Ainsi, en supposant un grand nombre de mineurs économiquement intéressés, un attaquant peut être en mesure d'envoyer une transaction en échange d'un bien numérique (généralement une autre crypto-monnaie), de recevoir le bien, puis de démarrer un fork de la blockchain à partir d'un bloc derrière la transaction et envoyer l'argent à eux-mêmes à la place, et même avec 1% de la mise totale, le fork de l'attaquant gagnerait parce que tout le monde exploite les deux.

**Long-range Attack**: In a naively implemented proof of stake, suppose that there is an attacker with 1% of all coins at or shortly after the genesis block. That attacker then starts their own chain, and starts mining it. Although the attacker will find themselves selected for producing a block only 1% of the time, they can easily produce 100 times as many blocks, and simply create a longer blockchain in that way.
