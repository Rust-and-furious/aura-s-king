# 🏰 Jeu de Rôle Textuel — "Aura Farming Simulator"

Vous êtes le protagoniste de cette histoire, un paysan dont la lignée est connue depuis sept générations pour une seule chose : la culture intensive de légumes oubliés. Saisi au début du jeu (ou appelé **Jean-Michel** par défaut si vous manquez d'inspiration), votre famille a fourni au royaume assez de navets pour nourrir une armée, mais n'a jamais reçu en retour qu'une dette fiscale et des ampoules aux mains.

Le Royaume est dirigé par le Roi Anthony, un souverain dont la bonté n'a d'égale que son besoin viscéral d'être impressionné. Le pays est saturé de héros en armure étincelante, de bardes célèbres et de magiciens excentriques. Pour sortir de votre condition et devenir enfin "Chevalier", il ne suffit pas d'être courageux. Il faut être légendaire.

Votre unique monnaie d'échange est l'Aura : une mesure abstraite de votre prestance, de votre renommée et de votre capacité à ne pas avoir l'air d'un paysan quand vous entrez dans une pièce. Vos points d'Aura peuvent grimper ou s'effondrer de manière totalement démesurée à la moindre action.

⏱️ **La Chronologie et la Gestion du Temps**
*   **La Journée de l'Aventurier** : Votre quête commence à **08h00** le matin et le Grand Bal d'adoubement du Roi Anthony se termine à précisément **20h00** le soir. 
*   **Les Ticks (Minutes)** : Pour modéliser cela dans le moteur Rust, les ticks représentent des **minutes écoulées** depuis 08h00 :
    *   **Heure de départ (08h00)** : `current_tick = 0`
    *   **Heure limite (20h00)** : `max_ticks = 720` (12 heures $\times$ 60 minutes).
    *   **Calcul de l'heure en jeu** : `Heure = 08:00 + (current_tick / 60)h : (current_tick % 60)m`.
*   **Échec Temporel** : Si `current_tick >= 720` avant d'être devant le trône, vous subissez un **GAME OVER CHRONOLOGIQUE** : *"Les portes du château se ferment. Le bal commence sans vous. Vous entendez les trompettes au loin alors que vous êtes encore dans la boue. Vous passerez le reste de votre vie à sarcler des navets sous la pluie."*

---

## 🛠️ Note Technique : Alignement Architecture (Moteur Rust)

Pour correspondre à l'architecture définie dans `AGENTS.md` :
1.  **Identifiants uniques (IDs JSON)** : Tous les interactables et les zones possèdent un identifiant symbolique sous forme de chaîne de caractères (ex: `"marmite_cuisine"`). Le moteur résout ces chaînes en indices physiques (`usize`) lors du chargement.
2.  **Mapping des Actions** : Toutes les interactions textuelles sont converties par le moteur en variantes de l'énumération technique `Action` (`Observer`, `Fouiller`, `Ramasser`, `Ouvrir`, `Fermer`, `Utiliser`, `Attaquer`, `Dialoguer`, `Deplacer`).
3.  **Gestion de l'Inventaire** : "Prendre" ou "Ramasser" un objet se traduit techniquement par le transfert de l'ID de l'entité de la liste `Zone.interactables` vers la liste `Player.inventory`.

---

## 📊 Configuration Initiale du Joueur
*   **Nom par défaut** : `"Jean-Michel"` (si non spécifié)
*   **Zone de départ** : `zone_maison`
*   **Aura de départ** : `0.0`
*   **Inventaire initial** : Vide `[]`
*   **Objectif d'Aura pour adoubement garanti** : `1 000 000.0 Aura` !

---

## 🏠 MAISON DU PAYSAN (Zone ID : `zone_maison`)
> *"Vous vous réveillez sur une paillasse qui gratte, dans une pièce où ça sent le chou et les regrets. Les regrets d'une vie passée le dos cambré à planter des navets. Aujourd'hui, c'est décidé : vous allez devenir chevalier. Pour retrouver votre honneur et enfin pouvoir redresser le dos. Reste à trouver comment sortir de chez vous."*

### Interactables :

#### **Lit** (ID JSON : `lit_joueur`)
*   **Action `Utiliser` (S'endormir)** : Consomme **60 à 120 minutes** (1h à 2h aléatoire).
    *   *50% de chance* : Rêve héroïque (**+10 000.0 Aura**).
    *   *50% de chance* : Cauchemar de paysan : vous rêvez que vous êtes un chou cultivé par vous-même (**-30 000.0 Aura**).

#### **Marmite** (ID JSON : `marmite_cuisine`)
*   **Action `Observer` (Regarder dedans)** : *"Au fond de la soupe tiède, quelque chose brille... C'est la clé de votre propre porte. Oui, vous êtes ce genre de personne à cacher ses biens précieux dans des lieux insolites que vous-même oubliez au bout de 2 heures."* -> Débloque et fait apparaître l'entité `cle_maison` dans la zone (consomme **2 minutes**).
*   **Action `Ramasser` (Prendre avec soi)** : Transfère l'ID `marmite_cuisine` dans l'inventaire du joueur (consomme **2 minutes**).

#### **Clé de la maison** (ID JSON : `cle_maison` - *Infiltrée ou masquée initialement*)
*   **Action `Ramasser`** : Transfère l'ID `cle_maison` dans l'inventaire du joueur (consomme **1 minute**).

#### **Le Balai** (ID JSON : `balai_depart`)
*   **Action `Ramasser`** : Transfère l'ID `balai_depart` dans l'inventaire du joueur (consomme **1 minute**).

#### **Fenêtre** (ID JSON : `fenetre_maison`)
*(Implémente le trait `Openable`)*
*   **Action `Ouvrir`** : Rend la fenêtre ouverte (consomme **1 minute**). Décrit l'environnement extérieur miteux (la plaine, le village au loin).
*   **Action `Fermer`** : Rend la fenêtre fermée (consomme **1 minute**).
*   **Action `Deplacer { target_zone: "zone_plaine" }` (Sauter par la fenêtre)** : Consomme **10 minutes** (temps de chute et de récupération).
    *   *Si la fenêtre est ouverte (100% de chance de succès)* : Déplace le joueur vers la Plaine (**+15 000.0 Aura** pour cette sortie audacieuse).
    *   *Si la fenêtre est fermée (100% de chance d'échec)* : Déplace le joueur vers la Plaine (**-100 000.0 Aura** : *"Vous traversez la vitre. Votre dignité ne s'en remet pas, votre peau non plus. À chaque pas, les bouts de verre s'enfoncent un peu plus"*).

#### **Porte** (ID JSON : `porte_maison`)
*(Implémente les traits `Openable` et `Fightable`)*
*   **Action `Ouvrir`** :
    *   *Si verrouillée et que le joueur n'a pas `cle_maison`* : Échec (consomme **2 minutes**, **-5 000.0 Aura** : *"Vous poussez. Rien. Vous repoussez. Toujours rien. Humiliant. Même un âne mourant aurait fait mieux"*).
    *   *Si déverrouillée ou avec la clé* : Ouvre la porte (consomme **1 minute**).
*   **Action `Deplacer { target_zone: "zone_plaine" }` (Sortir)** : Consomme **10 minutes** (trajet vers la plaine).
    *   *Si ouverte* : Déplace le joueur vers la Plaine.
*   **Action `Attaquer { degats: 10 }` (Enfoncer la porte)** : (consomme **5 minutes**).
    *   *Si porte verrouillée* : Jet de réussite (50% de chance).
        *   *Succès* : Casse la porte, la rend ouverte (**+25 000.0 Aura** : *"Héroïque ! La porte cède sous votre force de taureau !"*).
        *   *Échec* : La porte résiste (**-15 000.0 Aura** : *"Votre épaule dit non et se déboîte légèrement"*).
    *   *Si porte déverrouillée* : Jet de réussite (100% de chance).
        *   *Succès* : La porte s'ouvre bruyamment (**-20 000.0 Aura** : *"Vous enfoncez une porte ouverte. Littéralement. Tout le monde vous regarde bizarrement"*).

---

## 🌾 PLAINE (Zone ID : `zone_plaine`)
> *"L'air frais vous frappe le visage. Devant vous, une plaine s'étend, parsemée de trucs plus ou moins intéressants. Au loin, un moulin tourne. Ou pas. Difficile à dire d'ici."*
> **Directions connectées** : Nord => Forêt (`zone_foret`) | Est => Le Lac (`zone_lac`) | Sud => Village (`zone_village`) | Ouest => Maison (`zone_maison`) (Déplacements = **15 minutes** de marche par trajet).

### Interactables :

#### **Puits** (ID JSON : `puits_plaine`)
*   **Action `Observer`** : *"C'est profond et sombre. Comme votre avenir si vous ne bougez pas."* (consomme **1 minute**).
*   **Action `Dialoguer` (Crier dans le puits)** : **+5 000.0 Aura** (*"L'écho vous répond 'CHEVALIEEEEER', vous êtes galvanisé"*, consomme **2 minutes**).
*   **Action `Utiliser` (Descendre dans le puits)** : Jet de réussite (50% de chance). Consomme **30 minutes** (remontée pénible et humide).
    *   *Succès* : Trouve l'objet `corde_puits` et l'ajoute à l'inventaire du joueur (**+20 000.0 Aura**).
    *   *Échec* : **-15 000.0 Aura** (*"Vous glissez et remontez trempé. Un crapaud vous juge en coassant"*).
*   **Action `Utiliser` (Jeter un objet dedans)** : Supprime l'objet sélectionné de l'inventaire du joueur (**-8 000.0 Aura** : *"Pourquoi avez-vous fait ça ? Sérieusement ? Le peuple taupe n'a pas besoin de ça !"*, consomme **3 minutes**).

#### **Épouvantail** (ID JSON : `epouvantail_plaine`)
*   **Action `Observer`** : *"Un épouvantail. Il a l'air plus chevaleresque que vous. C'est vexant."* (consomme **1 minute**).
*   **Action `Ramasser` (Voler son chapeau)** : Transfère l'ID `chapeau_epouvantail` de l'épouvantail à l'inventaire du joueur (**+15 000.0 Aura**, consomme **3 minutes**).
*   **Action `Attaquer { degats: 5 }` (Le défier en duel)** : **+25 000.0 Aura** (*"Les corbeaux sont impressionnés, vous avez gagné contre un bâton habillé"*, consomme **10 minutes**).
*   **Action `Dialoguer` (Lui parler)** : **-50 000.0 Aura** (*"Il ne répond pas. Évidemment. Vous venez de parler à un tas de paille devant un corbeau moqueur"*, consomme **5 minutes**).

---

### Points d'intérêt de la Plaine :

### 🌾 Moulin (InterestPoint ID : `moulin_plaine`)
> *"Le moulin tourne paresseusement. Le meunier est un homme large, couvert de farine, qui vous regarde approcher avec la méfiance de quelqu'un qui a déjà été volé par un paysan. Peut-être même vous."*

#### **Le Meunier** (ID JSON : `meunier_pnj`)
*   **Action `Dialoguer` (Parler)** : *"Encore un va-nu-pieds qui veut devenir chevalier ? Reviens quand t'auras de quoi payer."* (consomme **3 minutes**).
*   **Action `Dialoguer` (Demander du travail)** : Débloque une quête de livraison de sacs de farine au village (**+5 000.0 Aura** pour votre dévouement, consomme **5 minutes**).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Marmite (`marmite_cuisine`)* : Échange la marmite contre l'objet `farine_enchantee` (**+30 000.0 Aura** : *"Ah, une marmite ! Parfaite pour ma soupe. Tiens, prends cette farine enchantée !"*).
    *   *Si autre objet* : *"Qu'est-ce que tu veux que je fasse de ça ?"* (Pas d'effet).

#### **Meule de pierre** (ID JSON : `meule_moulin`)
*   **Action `Observer`** : *"Une grande meule. Elle tourne. C'est son truc, c'est une meule."*
*   **Action `Utiliser` (Mettre la main dedans)** : **-500 000.0 Aura** et consomme **60 minutes** (*"Mauvaise idée, très mauvaise idée. Le meunier doit appeler le guérisseur pour recoudre votre dignité"*).
*   **Action `Utiliser` (Essayer de la soulever)** : Jet de réussite (10% de chance). Consomme **15 minutes** d'effort surhumain.
    *   *Succès* : **+750 000.0 Aura** (*"Le meunier est bouche bée, exploit légendaire !"*).
    *   *Échec* : **-20 000.0 Aura** (*"Votre dos fait un bruit de branche sèche et s'en souviendra longtemps"*).
*   **Action `Utiliser` (Essayer de croquer dedans)** : **-80 000.0 Aura** (*"C'est une meule de pierre, pas de fromage, idiot. Vous avez encore plus l'air d'un paysan sans dents désormais"*, consomme **5 minutes**).

#### **Sacs de farine** (ID JSON : `sacs_farine`)
*   **Action `Observer`** : *"Des dizaines de sacs empilés. Ça sent le pain et le labeur."* (consomme **1 minute**).
*   **Action `Fouiller`** : Jet de détection (40% de chance de se faire surprendre par le meunier). Consomme **10 minutes**.
    *   *Non surpris (60%)* : Trouve `piece_monnaie` et l'ajoute à l'inventaire.
    *   *Surpris (40%)* : Le meunier vous attrape. **-30 000.0 Aura** et aucun butin.
*   **Action `Attaquer { degats: 1 }` (Éventrer un sac)** : **-50 000.0 Aura** (*"Le meunier hurle, vous êtes couvert de farine. Pas très chevaleresque"*, consomme **5 minutes**).

---

### 🏡 Maison de Michu (InterestPoint ID : `maison_michu_plaine`)
> *"La maison de votre voisine Michu. Elle a 847 ans, est la plus grande commère et connaît tous les ragots du royaume, et fait les meilleurs biscuits de la région."*

#### **Porte de Michu** (ID JSON : `porte_michu`)
*(Implémente le trait `Openable`)*
*   **Action `Ouvrir` (Frapper)** : Michu ouvre la porte (*"Oh, c'est toi gamin ! Entre donc !"*, consomme **2 minutes**). Permet d'accéder aux interactables intérieurs.
*   **Action `Attaquer { degats: 10 }` (Enfoncer)** : Échec automatique (100% de chance). **-150 000.0 Aura** et consomme **120 minutes** d'évanouissement (*"Michu vous assomme d'un coup de poêle. Elle a de sacrés réflexes pour 847 ans. Vous vous réveillez deux heures plus tard..."*).

#### **Michu** (ID JSON : `michu_pnj` - *Disponible si porte ouverte*)
*   **Action `Dialoguer` (Parler)** : *"Le roi ? Ah oui, il adoube les mardis et jeudis. Faut prendre rendez-vous. Et surtout, faut pas sentir le chou."* (consomme **5 minutes**).
*   **Action `Dialoguer` (Demander conseil)** : Révèle des indices narratifs sur la forêt et le lac (consomme **5 minutes**).
*   **Action `Dialoguer` (Demander un biscuit)** : (consomme **3 minutes**).
    *   *Première fois* : Donne `biscuit_michu` à l'inventaire (**+10 000.0 Aura** : *"Les biscuits de Michu donnent du courage"*).
    *   *Deuxième fois et plus* : *"C'est pas un buffet ici !"* (Aucun effet).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Chapeau de l'épouvantail (`chapeau_epouvantail`)* : Échange contre `broche_michu` (**+50 000.0 Aura** : *"Oh ! Je cherchais ce chapeau pour mes poules depuis des années ! Tiens, prends cette broche"*).
    *   *Si autre objet* : *"C'est gentil mais non merci. Je ne suis pas Emmaüs."*

#### **Chat de Michu (nommé Pataud)** (ID JSON : `chat_michu` - *Disponible si porte ouverte*)
*   **Action `Utiliser` (Caresser)** : Jet de réussite (70% de chance, consomme **5 minutes**).
    *   *Succès* : **+8 000.0 Aura** (*"Pataud ronronne bruyamment, vous vous sentez validé"*).
    *   *Échec* : **-15 000.0 Aura** (*"Pataud n'est pas d'humeur et vous griffe méchamment le nez"*).
*   **Action `Utiliser` (Soulever)** : **-40 000.0 Aura** (*"Le chat se transforme en tornade de griffes. Michu vous gronde. Double peine."*, consomme **5 minutes**).
*   **Action `Dialoguer` (Parler au chat)** : *"Miaou."* (Pas d'effet, consomme **2 minutes**).

---

## 🌲 FORÊT (Zone ID : `zone_foret`)
> *"Les arbres se referment autour de vous comme les bras d'une belle-mère insistante. Il fait sombre, ça craque de partout, et vous êtes à peu près sûr que quelque chose vous observe. C'est un écureuil."*
> **Directions connectées** : Sud => Plaine (`zone_plaine`) | Est => Le Lac (`zone_lac`) | Nord => Cimetière (`zone_cimetiere`) (Déplacements = **20 minutes** de marche par trajet).

### Interactables :

#### **Panneau en bois** (ID JSON : `panneau_foret`)
*   **Action `Observer` (Lire)** : *"Bienvenue en Forêt de Brâme. Interdiction de crier, chanter, ou de de devenir chevalier sans permis."* (consomme **1 minute**).
*   **Action `Ramasser` (Arracher)** : Ajoute l'objet `planche_bois` à l'inventaire (**+15 000.0 Aura** : *"Rebelle dans l'âme !"*, consomme **5 minutes**). Supprime le panneau de la zone.

#### **Champignon suspect** (ID JSON : `champignon_foret`)
*   **Action `Observer`** : *"Il est violet, brillant et vibre légèrement. Tout va bien."* (consomme **1 minute**).
*   **Action `Utiliser` (Manger)** : Jet aléatoire (50/50).
    *   *Effet 1 (50%)* : Vision mystique (**+100 000.0 Aura**, consomme **10 minutes**).
    *   *Effet 2 (50%)* : Intoxication (**-50 000.0 Aura**, consomme **90 minutes** (1h30) de jeu : *"Vous parlez aux arbres. Ils ne répondent pas. Vous vomissez votre chou et reprenez vos esprits dans une mare"*).
*   **Action `Ramasser` (Cueillir)** : Ajoute `champignon_suspect` à l'inventaire (consomme **3 minutes**).

---

### Points d'intérêt de la Forêt :

### 🌳 Vieux Chêne (InterestPoint ID : `vieux_chene_foret`)
> *"Un chêne titanesque se dresse devant vous. Il est si vieux qu'il a probablement vu le premier roi du royaume se prendre les pieds dans sa cape."*

#### **Le Chêne** (ID JSON : `vieux_chene`)
*   **Action `Observer`** : *"Le tronc fait dix fois votre tour de taille. Ce qui n'est pas un exploit vu ce que vous mangez."* (consomme **1 minute**).
*   **Action `Utiliser` (Grimper)** : Jet de réussite (60% de chance). Consomme **30 minutes** (effort pénible).
    *   *Succès* : **+150 000.0 Aura** et ajoute `oeuf_dore` à l'inventaire (*"La vue sur le royaume est magnifique ! Un moment digne des chansons de geste ! Et vous trouvez un œuf doré dans un nid !"*).
    *   *Échec* : **-25 000.0 Aura** (*"Vous tombez dans un buisson de ronces. Un écureuil vient casser sa noisette sur votre front"*).
*   **Action `Utiliser` (Enlacer l'arbre)** : **+5 000.0 Aura** (*"C'est étrange mais étonnamment réconfortant"*, consomme **5 minutes**).
*   **Action `Utiliser` (Graver son nom)** : **+15 000.0 Aura** (*"Votre légende commence à s'inscrire dans l'écorce"*, consomme **10 minutes**).

#### **L'Ermite** (ID JSON : `ermite_pnj`)
*   **Action `Dialoguer` (Parler)** : *"Mmh ? Un paysan ? Je suis un ancien chevalier. J'ai tout quitté pour vivre dans cet arbre. Meilleure décision de ma vie."* (consomme **5 minutes**).
*   **Action `Dialoguer` (Demander conseil)** : Révèle : *"Règle numéro un : aie toujours l'air sûr de toi, même quand tu ne sais pas ce que tu fais. Surtout quand tu ne sais pas."* (**+10 000.0 Aura**, consomme **5 minutes**).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Champignon suspect (`champignon_foret`)* : Échange contre `medaille_ermite` (**+80 000.0 Aura** : *"AH ! Mon champignon ! Ça fait 12 ans que j'en cherche ! Tiens, prends cette médaille"*).
    *   *Si Marmite (`marmite_cuisine`)* : Échange contre `talisman_ermite` (**+40 000.0 Aura** : *"Je peux en faire une casserole. Tiens, un talisman protecteur en échange"*).
    *   *Si autre objet* : *"Non merci, la nature me fournit le nécessaire."*

---

### 💀 LE CIMETIÈRE DES CHEVALIERS RATÉS (Zone ID : `zone_cimetiere`)
> *"Vous arrivez dans un lieu brumeux et sinistre. Ici reposent ceux qui, comme vous, ont cru qu'un peu d'aura et une épée rouillée suffisaient pour impressionner le roi. L'herbe est morte, et votre moral s'apprête à faire de même."*
> **Directions connectées** : Sud => Forêt (`zone_foret`) (Déplacement = **20 minutes** de marche).

### Interactables :

#### **Tombe fraîchement creusée** (ID JSON : `tombe_fraiche`)
*   **Action `Observer`** : *"Il n'y a pas de nom. Mais les dimensions correspondent curieusement à votre taille et à votre carrure. C'est sûrement une coïncidence."* (consomme **1 minute**).
*   **Action `Utiliser` (S'allonger dedans)** : **-30 000.0 Aura** et consomme **60 minutes** (1h de sieste macabre : *"Vous testez le confort. C'est ferme. Vous perdez un temps précieux à déprimer au fond d'un trou"*).
*   **Action `Fouiller` (Fouiller la terre)** : Jet de réussite (50% de chance). Consomme **15 minutes**.
    *   *Succès* : Ajoute `ver_de_terre` à l'inventaire (**+10 000.0 Aura**).
    *   *Échec* : **-15 000.0 Aura** (*"Vous vous mettez de la terre dans l'œil. Félicitations."*).

#### **Le Fossoyeur** (ID JSON : `fossoyeur_pnj`)
*   **Action `Dialoguer` (Parler)** : *"Encore un futur client... Prends un ticket, j'suis débordé."* (consomme **3 minutes**).
*   **Action `Dialoguer` (Demander une pelle)** : *"Une pelle, ça se mérite. Ou ça s'achète. T'as l'air d'avoir ni l'un ni l'autre."* (consomme **3 minutes**).

---

### Points d'intérêt du Cimetière :

### 👻 Le Mausolée de Sire Godefroy (InterestPoint ID : `mausolee_godefroy`)
> *"Un grand bâtiment en pierre, couvert de statues qui ont l'air de vous juger. La porte est entrouverte, laissant échapper un courant d'air glacial."*

#### **Statue de Gargouille** (ID JSON : `gargouille_statue`)
*(Implémente le trait `Fightable`)*
*   **Action `Observer`** : *"Elle est très laide. Elle vous rappelle vaguement votre oncle Maurice."* (consomme **1 minute**).
*   **Action `Dialoguer` (Insulter la gargouille)** : **+5 000.0 Aura** (*"Ça fait du bien de se défouler, et puis elle ne va pas répondre... N'est-ce pas ?"*, consomme **3 minutes**).
*   **Action `Attaquer { degats: 10 }` (Essayer de la casser)** : Consomme **10 minutes**.
    *   *Si le joueur a `bidule_metal` ou `balai_depart` dans son inventaire* : Jet de réussite (30% de chance).
        *   *Succès* : La gargouille se brise, révélant un `rubis_rutilant` ajouté à l'inventaire (**+100 000.0 Aura**).
        *   *Échec* : L'outil rebondit. **-25 000.0 Aura** et l'objet utilisé se brise (supprimé de l'inventaire).
    *   *Si aucun outil* : Échec automatique (0% de chance). **-10 000.0 Aura** (*"Frapper de la pierre à mains nues... Vos poignets pleurent"*).

#### **Esprit de Sire Godefroy** (ID JSON : `esprit_godefroy`)
*   **Action `Dialoguer` (Parler)** : *"QUI OSE TROUBLER MON REPOS ? Oh, un bouseux. Pars, avant que je ne te maudisse avec une haleine d'ail éternelle."* (consomme **5 minutes**).
*   **Action `Dialoguer` (Demander sa mort)** : *"J'ai glissé sur une poule pendant mon adoubement. Mon crâne a rencontré le trône. Un complot, j'en suis sûr."* (Indice : le roi déteste les maladroits, consomme **5 minutes**).
*   **Action `Dialoguer` (Provoquer en duel d'Aura)** : (consomme **10 minutes**).
    *   *Si Aura du Joueur >= 150 000.0 (100% de chance)* : **+300 000.0 Aura** et donne le `manuel_chevalier` à l'inventaire (*"Le fantôme bégaie, impressionné par votre prestance, et s'évapore en vous laissant son manuel du Parfait Petit Chevalier !"*).
    *   *Si Aura du Joueur < 150 000.0 (0% de chance)* : **-100 000.0 Aura** (*"Il se moque de vous avec un rire d'outre-tombe. Votre ego est pulvérisé"*).

---

### 🦊 Clairière (InterestPoint ID : `clairiere_foret`)
> *"Un cercle d'herbe parfaitement tondu au milieu de la forêt. C'est suspect. Au centre, une souche avec quelque chose dessus."*

#### **Souche mystérieuse** (ID JSON : `souche_epee`)
*   **Action `Observer`** : *"Sur la souche, une épée est plantée. Elle est rouillée, tordue et franchement pas terrible. Mais c'est une ÉPÉE."* (consomme **1 minute**).
*   **Action `Utiliser` (Tirer l'épée)** : Jet de réussite (20% de chance). Consomme **15 minutes** d'effort dorsal.
    *   *Succès* : Ajoute `epee_rouillee` à l'inventaire (**+120 000.0 Aura** : *"VOUS AVEZ TIRÉ L'ÉPÉE DE LA SOUCHE ! Bon, ce n'est pas Excalibur, et prévoyez peut-être un vaccin contre le tétanos"*).
    *   *Échec* : **-20 000.0 Aura** (*"L'épée ne bouge pas. Vous vous êtes fait un tour de rein. Aïe"*).
*   **Action `Utiliser` (S'asseoir sur la souche)** : **+8 000.0 Aura** (*"Vous méditez un instant. Un papillon se pose sur votre nez. Très poétique."*, consomme **10 minutes**).

#### **Le Renard** (ID JSON : `renard_clairiere`)
*   **Action `Observer`** : *"Un renard roux vous fixe avec une intelligence dérangeante."* (consomme **1 minute**).
*   **Action `Utiliser` (Approcher doucement)** : Ajoute `baie_mysterieuse` à l'inventaire (**+15 000.0 Aura** : *"Le renard s'approche, renifle votre main et dépose une baie à vos pieds"*, consomme **5 minutes**).
*   **Action `Deplacer { target_zone: "zone_foret" }` (Courir après)** : **-10 000.0 Aura** (Consomme **20 minutes** car vous quittez la clairière pour courir dans les bois : *"Il court bien plus vite que vous et se retourne pour vous regarder avec mépris"*).
*   **Action `Dialoguer` (Parler au renard)** : *"Le renard penche la tête. Vous croyez qu'il comprend. Il ne comprend pas."* (Pas d'effet, consomme **3 minutes**).

---

## 🌊 LE LAC (Zone ID : `zone_lac`)
> *"Un lac d'un bleu étrangement parfait s'étale devant vous. La surface est lisse comme un miroir. Vous vous y voyez. Vous détournez le regard."*
> **Directions connectées** : Ouest => Plaine (`zone_plaine`) | Nord => Forêt (`zone_foret`) | Sud => Village (`zone_village`) (Déplacements = **15 minutes** de marche par trajet).

### Interactables :

#### **Le Lac** (ID JSON : `eau_lac`)
*   **Action `Observer` (Regarder son reflet)** : *"Vous voyez un paysan. Mais si vous plissez les yeux... non, c'est toujours un paysan."* (consomme **1 minute**).
*   **Action `Utiliser` (Se baigner)** : **+15 000.0 Aura** et consomme **30 minutes** (*"Bain rafraîchissant, vous sentez moins le chou pour l'instant"*).
*   **Action `Utiliser` (Boire l'eau)** : **+5 000.0 Aura** (*"L'eau est fraîche et pure. Vous vous sentez revigoré"*, consomme **3 minutes**).
*   **Action `Utiliser` (Jeter un objet)** : (consomme **3 minutes**).
    *   *Si Pièce de monnaie (`piece_monnaie`)* : Échange la pièce contre **+80 000.0 Aura** (*"L'eau brille, une voix murmure : 'Merci, ça faisait longtemps'"*).
    *   *Si autre objet* : L'objet est supprimé de l'inventaire. **-25 000.0 Aura** (*"Plouf. Ça coule. Bravo, vous polluez la nature"*).

#### **Vieille barque** (ID JSON : `barque_lac`)
*   **Action `Observer`** : *"Une barque avec un trou béant dans la coque. Classique."* (consomme **1 minute**).
*   **Action `Utiliser` (Monter dedans sans réparer)** : **-40 000.0 Aura** (*"Vous coulez lentement en essayant de garder votre dignité. L'eau est froide"*, consomme **10 minutes**).
*   **Action `Utiliser` (Réparer la barque)** : Consomme **30 minutes** de bricolage.
    *   *Si le joueur a `planche_bois` et `corde_puits`* : Répare la barque (**+30 000.0 Aura** : *"C'est pas joli, mais ça flotte !"*). Débloque l'accès à l'île.
*   **Action `Deplacer { target_zone: "zone_ile" }` (Prendre la barque)** : Consomme **10 minutes** (traversée à la rame).
    *   *Si réparée* : Déplace le joueur vers l'Île.

---

### Points d'intérêt du Lac :

### 🏝️ Île au milieu du lac (InterestPoint ID : `ile_lac` - *Requiert barque réparée*)
> *"Une minuscule île avec un unique arbre tordu et un coffre recouvert de mousse. On dirait la planque d'un pirate qui avait un très petit budget."*

#### **Coffre moussu** (ID JSON : `coffre_ile`)
*(Implémente le trait `Openable`)*
*   **Action `Observer`** : *"Un coffre en bois solide. Le cadenas est rouillé."* (consomme **1 minute**).
*   **Action `Ouvrir` (Forcer le cadenas)** : Jet de réussite (40% de chance, consomme **10 minutes**).
    *   *Succès* : Ouvre le coffre. Révèle la `cape_brodee` ajoutée à l'inventaire (**+150 000.0 Aura**).
    *   *Échec* : **-40 000.0 Aura** (*"Vos doigts saignent sur la rouille"*).
*   **Action `Attaquer { degats: 10 }` (Frapper avec un objet)** : (consomme **5 minutes**).
    *   *Si Épée rouillée (`epee_rouillee`)* : Ouvre le coffre, détruit l'épée de l'inventaire (*"L'épée se brise mais le cadenas aussi. Marché conclu."*). Donne la `cape_brodee` (**+150 000.0 Aura**).
    *   *Si Marmite (`marmite_cuisine`)* : Ouvre le coffre, détruit la marmite (*"BONG ! Le bruit résonne sur tout le lac. Le coffre cède"*). Donne la `cape_brodee` (**+100 000.0 Aura**).
    *   *Si autre objet* : *"Ça fait 'toc'. Le coffre s'en fiche."* (Pas d'effet).

#### **Arbre tordu** (ID JSON : `arbre_tordu_ile`)
*   **Action `Utiliser` (Grimper)** : **+20 000.0 Aura** (*"La vue sur le château au loin est superbe. Vous vous y croyez déjà"*, consomme **15 minutes**).
*   **Action `Utiliser` (Secouer)** : Ajoute `noix_de_coco` à l'inventaire (**+5 000.0 Aura** : *"Une noix de coco tombe. Vous n'êtes même pas sous les tropiques. Ne cherchez pas"*, consomme **3 minutes**).

---

### 🎣 Ponton de pêche (InterestPoint ID : `ponton_lac`)
> *"Un ponton branlant avance sur le lac. Un seau vide, une canne à pêche cassée, et une odeur de poisson qui date d'un autre siècle."*

#### **Canne à pêche** (ID JSON : `canne_peche`)
*   **Action `Observer`** : *"Cassée en deux. Comme vos rêves. Mais les rêves, ça se répare."* (consomme **1 minute**).
*   **Action `Utiliser` (Réparer la canne)** :
    *   *Si le joueur a `corde_puits`* : Répare la canne (**+20 000.0 Aura**, consomme **10 minutes**). Débloque l'action Pêcher.
*   **Action `Utiliser` (Pêcher)** : Jet aléatoire (uniquement si réparée). Consomme **30 minutes** de pêche silencieuse.
    *   *40% chance (Poisson)* : Ajoute `poisson_frais` à l'inventaire (**+30 000.0 Aura**).
    *   *30% chance (Botte)* : Ajoute `vieille_botte` à l'inventaire (**+2 000.0 Aura** : *"Une botte taille 47. Inutile mais amusant"*).
    *   *30% chance (Rien)* : Aucun effet (*"Les poissons sont au courant de votre condition sociale et ignorent l'appât"*).

#### **Seau** (ID JSON : `seau_ponton`)
*   **Action `Ramasser`** : Ajoute `seau_vide` à l'inventaire (consomme **2 minutes**).
*   **Action `Utiliser` (Mettre sur la tête)** : **-50 000.0 Aura** (*"Vous ne voyez plus rien, trébuchez et tombez dans le lac. Bravo"*, consomme **5 minutes**).

---

## 🏘️ LE VILLAGE (Zone ID : `zone_village`)
> *"Le village de Bourg-les-Navets s'anime devant vous. Trois maisons, une taverne, une forge et un château qui essaie très fort d'être impressionnant. Des poules se promènent avec plus d'assurance que vous."*
> **Directions connectées** : Nord => Plaine (`zone_plaine`) | Est => Le Lac (`zone_lac`) (Déplacements = **15 minutes** de marche par trajet).

### Interactables :

#### **Poules** (ID JSON : `poules_village`)
*   **Action `Utiliser` (Caresser)** : **+5 000.0 Aura** (*"La poule accepte. C'est doux. Vous repensez brièvement à vos choix de vie"*, consomme **2 minutes**).
*   **Action `Deplacer { target_zone: "zone_village" }` (Courir après)** : **-30 000.0 Aura** (Consomme **15 minutes** de course fatigante : *"Tout le village vous regarde. Les poules sont bien plus rapides. Vous avez l'air ridicule"*).
*   **Action `Dialoguer` (Parler aux poules)** : *"Cot. Cot cot. Cot."* (Pas d'effet, consomme **2 minutes**).

#### **Fontaine** (ID JSON : `fontaine_village`)
*   **Action `Utiliser` (Boire)** : **+5 000.0 Aura** (*"L'eau est tiède et a un goût de calcaire. C'est la meilleure eau que vous ayez bue."*, consomme **2 minutes**).
*   **Action `Utiliser` (Jeter une pièce)** : (consomme **2 minutes**).
    *   *Si le joueur a `piece_monnaie`* : Supprime la pièce de l'inventaire. **+40 000.0 Aura** (*"Vous faites le vœu d'avoir de l'aura. Méta."*).
*   **Action `Utiliser` (Se laver)** : **+15 000.0 Aura** (*"Vous sentez moins le chou. Les villageois vous regardent avec un dégoût modéré"*, consomme **15 minutes**).

#### **Le Marchand** (ID JSON : `marchand_pnj`)
*   **Action `Dialoguer` (Parler / Acheter)** : (consomme **5 minutes**).
    *   *Acheter le Philtre de Charisme Absolu* : Coûte `piece_monnaie`. Si acheté, donne `philtre_charisme` à l'inventaire.
        *   *(Note de gameplay sur le philtre)* : Si le joueur utilise `philtre_charisme` via `Utiliser` -> **-100 000.0 Aura** et consomme **60 minutes** (1h) de colique carabinée (*"C'était de l'eau du lac et du jus de chou. Vous êtes malade"*).
    *   *Échanger le Rubis Rutilant* : Si le joueur donne `rubis_rutilant`, le marchand l'échange contre `armure_rutilante` (Une fois dans l'inventaire, donne **+500 000.0 Aura** immédiats !).

---

### Points d'intérêt du Village :

### 🍺 Taverne "Au Cochon Pendu" (InterestPoint ID : `taverne_village`)
> *"La taverne sent la bière renversée et les décisions regrettables. Un barde chante faux dans un coin. Le tavernier essuie un verre qui n'a jamais été propre de sa vie."*

#### **Le Tavernier** (ID JSON : `tavernier_pnj`)
*   **Action `Dialoguer` (Parler)** : *"Bienvenue au Cochon Pendu ! On sert de la bière, des rumeurs et des mauvais conseils."* Indique que le roi cherche un remplaçant pour son bouffon démissionnaire. (consomme **3 minutes**).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **3 minutes**).
    *   *Si Poisson (`poisson_frais`)* : Échange contre **+50 000.0 Aura** (*"Un poisson frais ! Ça change du ragoût éternel. Tiens, bois un coup à ma santé"*).
    *   *Si Noix de coco (`noix_de_coco`)* : Échange contre **+30 000.0 Aura** (*"C'est quoi ce truc ? ...On va en faire un cocktail."*).

#### **Le Barde** (ID JSON : `barde_pnj`)
*   **Action `Dialoguer` (Écouter chanter)** : **-15 000.0 Aura** (*"C'est vraiment très mauvais. Vos oreilles saignent"*, consomme **10 minutes**).
*   **Action `Dialoguer` (Demander une chanson sur vous)** : Jet de réussite (40% de chance, consomme **15 minutes**).
    *   *Succès* : **+200 000.0 Aura** (*"La chanson est atroce mais entraînante, les gens scandent votre nom !"*).
    *   *Échec* : **-80 000.0 Aura** (*"Le barde improvise sur 'Le paysan qui pue le chou'. Humiliation publique."*).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Œuf doré (`oeuf_dore`)* : Échange l'œuf contre **+400 000.0 Aura** (*"PAR LES DIEUX ! Un œuf de phénix ! Je compose un OPÉRA entier en votre honneur !"*).
    *   *Si Vieille botte (`vieille_botte`)* : Échange contre **+15 000.0 Aura** (*"Je peux en faire un instrument percussif bizarre. Merci !"*).

#### **Tonneau de la taverne** (ID JSON : `tonneau_taverne`)
*   **Action `Observer`** : *"Un tonneau entrouvert. Ça sent fort le fermenté."* (consomme **1 minute**).
*   **Action `Utiliser` (Boire dedans)** : Jet de chance (50% Bon / 50% Mauvais). Consomme **10 minutes**.
    *   *Bon (50%)* : **+30 000.0 Aura** (*"Un cidre vigoureux !"*).
    *   *Mauvais (50%)* : **-40 000.0 Aura** et consomme **60 minutes** d'évanouissement gastrique (*"Du vinaigre. De l'ancien vinaigre très acide"*).
*   **Action `Utiliser` (Se cacher dedans)** : **+5 000.0 Aura** (*"Vous vous y cachez. Personne ne vous cherchait de toute façon"*, consomme **15 minutes**).

---

### ⚒️ La Forge (InterestPoint ID : `forge_village`)
> *"La chaleur vous frappe comme une gifle. Le forgeron, un colosse tatoué, tape sur une enclume avec la passion de quelqu'un qui règle des comptes personnels avec le métal."*

#### **Le Forgeron** (ID JSON : `forgeron_pnj`)
*   **Action `Dialoguer` (Parler)** : *"J'forge. Tu veux quoi ?"* (consomme **3 minutes**).
*   **Action `Dialoguer` (Demander une armure)** : *"T'as de quoi payer ? Non ? Alors dégage... Ou ramène-moi du bon métal."* (consomme **3 minutes**).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Épée rouillée (`epee_rouillee`)* : Échange contre `epee_reforgee` (**+150 000.0 Aura** : *"Oh ! Du bon acier sous la rouille ! Tiens, je l'ai reforgee"*).
    *   *Si Médaille de l'ermite (`medaille_ermite`)* : Échange contre `bouclier_fer` (**+100 000.0 Aura** : *"Un bon métal ancien. Tiens, je t'ai fait un bouclier en échange"*).

#### **Enclume** (ID JSON : `enclume_forge`)
*   **Action `Observer`** : *"Lourde. Très lourde. Parfaitement enclume."* (consomme **1 minute**).
*   **Action `Utiliser` (Demander à utiliser la forge)** : Jet de réussite (30% de chance). Consomme **30 minutes** de travail du métal.
    *   *Succès* : Ajoute `bidule_metal` à l'inventaire (**+30 000.0 Aura**).
    *   *Échec* : **-15 000.0 Aura** (*"Vous vous brûlez au second degré. Le forgeron soupire"*).
*   **Action `Utiliser` (Essayer de soulever)** : Jet de réussite (5% de chance, consomme **5 minutes**).
    *   *Succès* : **+900 000.0 Aura** (*"L'EXPLOIT ! Le village entier vous acclame en héros !"*).
    *   *Échec* : **-30 000.0 Aura** (*"Non, votre colonne vertébrale refuse catégoriquement"*).
*   **Action `Attaquer { degats: 1 }` (Frapper dessus avec la Marmite)** : **-15 000.0 Aura** (*"BONG ! La marmite est cabossée. Le forgeron vous jette un regard noir"*, consomme **3 minutes**).

---

### 🏰 Le Château du Roi (Zone ID : `zone_chateau`)
> *"Le château se dresse devant vous, majestueux et... légèrement de travers ? Le pont-levis grince. Les gardes ont l'air de s'ennuyer profondément."*

#### **Gardes** (ID JSON : `gardes_chateau`)
*(Implémentent le trait `Fightable`)*
*   **Action `Dialoguer` (Parler / Demander à entrer)** : (consomme **5 minutes**).
    *   *Si le joueur a `cape_brodee` ou `armure_rutilante` dans son inventaire* : Accès autorisé à la salle du trône (`zone_salle_trone`).
    *   *Sinon* : Accès refusé (*"Reviens quand t'auras l'air de quelqu'un d'important"*).
*   **Action `Dialoguer` (Corrompre)** : (consomme **5 minutes**).
    *   *Si le joueur donne `piece_monnaie` ou `rubis_rutilant`* : Accès autorisé (**+20 000.0 Aura**).
*   **Action `Attaquer { degats: 10 }` (Forcer le passage)** : Jet de réussite (10% de chance). Consomme **15 minutes**.
    *   *Succès* : Vous passez en force (**+150 000.0 Aura**). Débloque la zone `zone_salle_trone`.
    *   *Échec* : **-100 000.0 Aura** et consomme **60 minutes** (1h) (*"Les gardes vous plaquent au sol en 0.3 secondes. Votre visage goûte la poussière pendant votre garde à vue"*).

#### **Pont-levis** (ID JSON : `pont_levis`)
*   **Action `Deplacer { target_zone: "zone_salle_trone" }` (Traverser)** : Déplace le joueur vers la Salle du trône (uniquement si l'accès est déverrouillé). Consomme **5 minutes**.

---

### 👑 Salle du Trône (Zone ID : `zone_salle_trone` - *Zone Finale*)

#### **Le Roi Anthony** (ID JSON : `roi_anthony`)
*   **Action `Dialoguer` (Se présenter - ÉVALUATION FINALE)** :
    *   *Si Aura du Joueur >= 1 000 000.0* : **VICTOIRE ABSOLUE** (*"Le roi sourit et vous adoube ! SIR [JOUEUR] ! La foule applaudit, les poules et l'épouvantail aussi !"*).
    *   *Si Aura du Joueur < 1 000 000.0* : Jet de chance.
        *   *Succès (Chance = 20%)* : **VICTOIRE DE JUSTESSE** (*"Le roi hésite... mais votre culot légendaire lui plaît. Il vous adoube sur un coup de tête !"*).
        *   *Échec (Chance = 80%)* : **GAME OVER** (*"Le roi éclate de rire et vous fait jeter dehors. Retournez sarcler vos navets"*).
*   **Action `Dialoguer` (Offrir un objet)** : (consomme **5 minutes**).
    *   *Si Œuf doré (`oeuf_dore`)* : **+250 000.0 Aura** (*"Le roi adore ! Il l'installe sur son trône"*).
    *   *Si Talisman (`talisman_ermite`)* : **+100 000.0 Aura**.
    *   *Si Bidule en métal (`bidule_metal`)* : **+50 000.0 Aura** (*"C'est moche... J'adore ! Dit le roi"*).
    *   *Si Marmite (`marmite_cuisine`)* : **-80 000.0 Aura** (*"Des gardes pour ce récipient ?"*).
    *   *Si Seau (`seau_vide`)* : **-200 000.0 Aura** (*"Le roi prend cela pour une insulte royale"*).
