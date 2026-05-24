🏰 Jeu de Rôle Textuel — "Aura Farming Simulator"

Vous êtes **[Nom du Joueur]**, un paysan dont la lignée est connue depuis sept générations pour une seule chose : la culture intensive de légumes oubliés. Votre famille a fourni au royaume assez de navets pour nourrir une armée, mais n'a jamais reçu en retour qu'une dette fiscale et des ampoules aux mains.

Le Royaume est dirigé par le Roi Anthony, un souverain dont la bonté n'a d'égale que son besoin viscéral d'être impressionné. Le pays est saturé de héros en armure étincelante, de bardes célèbres et de magiciens excentriques. Pour sortir de votre condition et devenir enfin "Chevalier", il ne suffit pas d'être courageux. Il faut être légendaire.

Votre unique monnaie d'échange est l'Aura : une mesure abstraite de votre prestance, de votre renommée et de votre capacité à ne pas avoir l'air d'un paysan quand vous entrez dans une pièce.

⏱️ Les Contraintes du Destin
La Chronologie : Le Grand Bal du Roi Anthony aura lieu dans précisément X ticks (tours de jeu). Si vous n'êtes pas devant le trône avant la fin du décompte, vous finirez vos jours à sarcler des mauvaises herbes sous la pluie.
L'Aura est volatile : Un exploit héroïque peut vous apporter une aura immense, mais une humiliation publique (tomber dans la boue devant des poules, parler à un épouvantail) peut effacer des jours d'efforts.

## 🏠 MAISON DU PAYSAN (Zone de départ)

"Vous vous réveillez sur une paillasse qui gratte, dans une pièce où ça sent le chou et les regrets. Les regrets d'une vie passée le dos cambré a planter des navets"
--ajoute des details sur sa vie miserable
Aujourd'hui, c'est décidé : vous allez devenir chevalier. Pour retrouver votre honneur et enfin pouvoir redresser le dos. (trouver meilleurs tournure de phrase)
Reste à trouver comment sortir de chez vous."\*

**Interactables :**

**Lit**

- S'endormir => Skip des ticks de jeu, +/- aura _(parfois on rêve qu'on est chevalier, parfois on rêve qu'on est un chou cultivé par soi meme )_

**Marmite**

- Regarder dedans => _"Au fond de la soupe tiède, quelque chose brille... C'est la clé de votre propre porte. Oui vous êtes ce genre de personne a cacher ces biens precieux dans des lieux insolites que vous meme oublié au bout de 2 heures."_ => Donne la clé de la porte
- Prendre avec nous => Ajoute marmite à l'inventaire, la supprime de la zone

**Le Balai**

Ramasser => Ajoute Balai à l'inventaire.
**Fenêtre**

- Ouvrir => Décrit l'environnement extérieur miteux*(la plaine, le village au loin)*
- Sauter par la fenêtre =>
  - Si ouverte : +aura _(sortie audacieuse)_ => zone Plaine
  - Si fermée : -aura _(vous traversez la vitre, votre dignité ne s'en remet pas, votre peau non plus a chaque pas les bout de verre s'enfoncent de plus en plus )_ => zone Plaine

**Porte**

- Ouvrir =>
  - Si verrouillée et pas la clé : -aura _(vous poussez. Rien. Vous repoussez. Toujours rien. Humiliant.Meme un ane mourant aurai fait mieux)_
  - Si ouvrable : ouvre la porte, change état interne
    - Sortir _(si porte ouverte)_ => zone Plaine
- Enfoncer la porte =>
  - Succès + porte verrouillée : +aura _(héroïque !)_
  - Succès + porte pas verrouillée : -aura _(vous enfoncez une porte ouverte. Littéralement.)_
  - Échec + porte verrouillée : -aura _(l'épaule dit non)_
  - Échec + porte pas verrouillée : --aura _(vous ratez une porte ouverte. Impressionnant.)_
    - Sortir _(si porte ouverte)_ => zone Plaine

---

## 🌾 PLAINE

> _"L'air frais vous frappe le visage. Devant vous, une plaine s'étend, parsemée de trucs plus ou moins intéressants. Au loin, un moulin tourne. Ou pas. Difficile à dire d'ici."_

**Directions :** Nord => Forêt | Est => Le Lac | Sud => Village | Ouest => Maison du Paysan

### Interactables directs de la plaine :

**Puits**

- Regarder dedans => _"C'est profond et sombre. Comme votre avenir si vous ne bougez pas."_
- Crier dedans => +aura _(l'écho vous répond "CHEVALIEEEEER", vous êtes galvanisé)_
- Descendre => Jet de réussite
  - Succès : Trouve une corde au fond => ajout inventaire, +aura
  - Échec : -aura, _"Vous glissez, remontez trempé. Un crapaud vous juge."_
- Jeter un objet dedans _(si inventaire non vide)_ => Supprime l'objet, -aura _(pourquoi avez-vous fait ça ? serieusement ? le peuple taupe n'a pas besoin de ça!)_

**Épouvantail**

- Examiner => _"Un épouvantail. Il a l'air plus chevaleresque que vous. C'est vexant."_
- Voler son chapeau => Ajoute chapeau à l'inventaire, +aura _(premier butin, premier pas)_
- Le défier en duel => +aura _(les corbeaux sont impressionnés, vous avez gagné contre un bâton habillé)_
- Lui parler => -aura _(il ne répond pas. Évidemment. Vous avez parlé à un épouvantail.)_

---

### Points d'intérêt de la plaine :

### 🌾 Moulin

> _"Le moulin tourne paresseusement. Le meunier est un homme large, couvert de farine, qui vous regarde approcher avec la méfiance de quelqu'un qui a déjà été volé par un paysan.Peut etre meme vous"_

**Interactables :**

**Le Meunier (PNJ)**

- Parler => _"Encore un va-nu-pieds qui veut devenir chevalier ? Reviens quand t'auras de quoi payer."_
- Demander du travail => _"Porte ces sacs de farine au village. Je te donnerai quelque chose."_ => +aura (aider les autres, bravo tres chevalier de votre part)
- Offrir un objet =>
  - Marmite : +aura, _"Ah, une marmite ! Parfait pour ma soupe. Tiens, prends cette farine enchantée."_ => Ajoute farine enchantée à l'inventaire, supprime marmite
  - Autre objet : _"Qu'est-ce que tu veux que je fasse de ça ?"_

**Meule de pierre**

- Examiner => _"Une grande meule. Elle tourne. C'est son truc, c'est une meule"_
- Mettre la main dedans => -aura _(mauvaise idée, très mauvaise idée)_
- Essayer de la soulever => Jet de réussite
  - Succès : ++aura _(le meunier est bouche bée, exploit légendaire)_
  - Échec : -aura _(votre dos s'en souviendra)_
- Essayer de croquer dedans => --aura _"C'est une meule de pierre pas de fromage, idiot. Vous avez encore plus l'air d'un paysans maintenant sans dents"_

**Sacs de farine**

- Examiner => _"Des dizaines de sacs empilés. Ça sent le pain et le labeur."_
- Fouiller => _"Vous trouvez une pièce cachée entre deux sacs !"_ => Ajoute pièce à l'inventaire -chance de se faire prendre par le meunier (-aura)
- Éventrer un sac => -aura _(le meunier hurle, vous êtes couvert de farine, pas très chevaleresque)_

---

### 🏡 Maison de Michu

> _"La maison de votre voisine Michu. Elle a 847 ans, est la plus grande comere et connaît tous les ragots du royaume, et fait les meilleurs biscuits de la région."_

**Interactables :**

**Porte de Michu**

- Frapper => Michu ouvre, _"Oh, c'est toi gamin ! Entre donc !"_ => ouvre l'accès intérieur
- Enfoncer => -aura _(Michu vous assomme avec une poêle. Elle a de bons réflexes pour 847 ans.)_

**Michu (PNJ)** _(accessible si porte ouverte)_

- Parler => _"Le roi ? Ah oui, il adoube les mardis et jeudis. Faut prendre rendez-vous. Et surtout, faut pas sentir le chou."_
- Demander conseil => _"Tu veux de l'aura ? Va voir la forêt, y'a un vieux chêne qui impressionne les gens. Et fais gaffe au lac, y'a un truc louche dedans."_ => Révèle des indices sur les zones
- Demander un biscuit => +aura _(les biscuits de Michu donnent du courage)_
  - Deuxième fois : _"C'est pas un buffet ici !"_ => pas d'effet
- Offrir un objet =>
  - Chapeau de l'épouvantail : +aura, _"Oh ! Je cherchais ce chapeau depuis des années ! Tiens, prends cette broche."_ => Ajoute broche à l'inventaire
  - Autre : _"C'est gentil mais non merci. Je suis pas emaus"_

**Chat de Michu**

- Caresser
  Réussite => +aura _(le chat ronronne, vous vous sentez validé)_
  Echec => -aura _(trouver un nom drole pour le chat de Mme Michu) n'est pas content et vous griffe_
- Soulever => -aura _(le chat vous griffe. Michu vous gronde. Double peine.)_
- Parler au chat => _"Miaou."_ _(pas d'effet, mais c'était un moment agréable)_

---

## 🌲 FORÊT

> _"Les arbres se referment autour de vous comme les bras d'une belle-mère insistante et son menton qui pique . Il fait sombre, ça craque partout, et vous êtes à peu près sûr que quelque chose vous observe. C'est un écureuil."_

**Directions :** Sud => Plaine | Est => Le Lac

### Interactables directs de la forêt :

**Panneau en bois**

- Lire => _"Bienvenue en Forêt de Brâme. Interdiction de crier, chanter, ou devenir chevalier sans permis."_
- Arracher => +aura _(rebelle !)_ => Ajoute planche à l'inventaire
- Suivre la direction indiquée => Vers le Vieux Chêne

**Champignon suspect**

- Examiner => _"Il est violet, brillant et vibre légèrement. Tout va bien."_
- Manger => Jet aléatoire
  - Effet 1 : +aura _(vision mystique, vous voyez votre avenir de chevalier !)_
  - Effet 2 : -aura _(vous parlez aux arbres pendant 3 ticks. Les arbres ne répondent pas. Vomissez et repartez de la ou vous etes venu, bravo)_
- Cueillir => Ajoute champignon suspect à l'inventaire

---

### Points d'intérêt de la forêt :

### 🌳 Vieux Chêne

> _"Un chêne titanesque se dresse devant vous. Il est si vieux qu'il a probablement vu le premier roi du royaume se prendre les pieds dans sa cape."_

**Interactables :**

**Le Chêne lui-même**

- Examiner => _"Le tronc fait dix fois votre tour de taille. Ce qui n'est pas un exploit vu ce que vous mangez."_
- Grimper => Jet de réussite
  - Succès : ++aura _(vous voyez tout le royaume d'en haut ! Moment épique !)_ + Trouve un nid avec un œuf doré => ajout inventaire
  - Échec : -aura _(vous tombez dans un buisson. Un écureuil vient casser sa noisette sur votre front.)_
- Enlacer l'arbre => +aura _(c'est étrange mais réconfortant)_
- Graver son nom => +aura _(votre légende commence ici)_

**Ermite dans le tronc (PNJ)**

- Parler => _"Mmh ? Un paysan ? Je suis un ancien chevalier. J'ai tout quitté pour vivre dans cet arbre. Meilleure décision de ma vie."_
- Demander des conseils de chevalerie => _"Règle numéro un : aie toujours l'air sûr de toi, même quand tu ne sais pas ce que tu fais. Surtout quand tu ne sais pas."_ => +aura
- Offrir un objet =>
  - Champignon suspect : ++aura, _"AH ! Mon champignon ! Ça fait 12 ans que j'en cherche ! Tiens, prends cette médaille."_ => Ajoute médaille de l'ermite à l'inventaire
  - Marmite : +aura, _"Je peux en faire une armure... non, une casserole. Tiens, un talisman en échange."_ => Ajoute talisman à l'inventaire
  - Autre : _"Non merci, j'ai tout ce qu'il me faut dans mon arbre."_

---

### 🪦 LE CIMETIÈRE DES CHEVALIERS RATÉS

"Vous arrivez dans un lieu brumeux et sinistre. Ici reposent ceux qui, comme vous, ont cru qu'un peu d'aura et une épée rouillée suffisaient pour impressionner le roi. L'herbe est morte, et votre moral s'apprête à faire de même."

Directions : Nord => Le Village | Est => Les Marais (Bloqué)

Interactables directs du cimetière :
-Tombe fraîchement creusée

Examiner => "Il n'y a pas de nom. Mais les dimensions correspondent curieusement à votre taille et à votre carrure. C'est sûrement une coïncidence."

S'allonger dedans => -aura, + skip 2 ticks (vous testez le confort. C'est ferme. Vous perdez un temps précieux à déprimer.)

Fouiller le tas de terre => Jet de réussite

Succès : Trouve un Ver de terre charnu => ajout inventaire

Échec : "Vous vous mettez de la terre dans l'œil. Félicitations." -aura

-Fossoyeur dépressif (PNJ)

Parler => "Encore un futur client... Prends un ticket, j'suis débordé."

Demander une pelle => "Une pelle, ça se mérite. Ou ça s'achète. T'as l'air d'avoir ni l'un ni l'autre."

Points d'intérêt du cimetière :
👻 Le Mausolée de Sire Godefroy le Prétentieux
"Un grand bâtiment en pierre, couvert de statues qui ont l'air de vous juger. La porte est entrouverte, laissant échapper un courant d'air glacial."

Interactables :

Statue de Gargouille

Examiner => "Elle est très laide. Elle vous rappelle vaguement votre oncle Maurice."

Insulter la statue => +aura (ça fait du bien de se défouler, et puis de toute façon elle ne va pas répondre. N'est-ce pas ?, N'est-ce pas ?)

Essayer de la casser (Nécessite un outils) =>

Interaction (successRate faible) :

Succès : La gargouille se brise, révélant un Rubis rutilant => ajout inventaire, +aura.

Échec : L'arme rebondit. Durabilité de l'arme -1. -aura (vos poignets pleurent).

Esprit de Sire Godefroy (PNJ/Interactable)

Parler => "QUI OSE TROUBLER MON REPOS ? Oh, un bouseux. Pars, avant que je ne te maudisse avec une haleine d'ail éternelle."

Demander comment il est mort => "J'ai glissé sur une poule pendant mon adoubement. Mon crâne a rencontré le trône. Un complot, j'en suis sûr." => Indice sur le roi (le roi déteste les maladroits).

Provoquer en duel d'Aura => Interaction (Requiert un minimum d'Aura de base)

Succès (si votre aura est haute) : ++aura (Le fantôme bégaie, impressionné par votre prestance, et s'évapore en vous laissant son Manuel du Parfait Petit Chevalier). => Ajout Manuel.

Échec : --aura (Il se moque de vous avec un rire d'outre-tombe. Votre ego est pulvérisé).

### 🦊 Clairière

> _"Un cercle d'herbe parfaitement tondu au milieu de la forêt. C'est suspect. Au centre, une souche avec quelque chose dessus."_

**Interactables :**

**Souche mystérieuse**

- Examiner => _"Sur la souche, une épée est plantée. Elle est rouillée, tordue, et franchement pas terrible. Mais c'est une ÉPÉE."_
- Tirer l'épée => Jet de réussite
  - Succès : ++aura _(VOUS AVEZ TIRÉ L'ÉPÉE DE LA SOUCHE ! Bon, c'est pas Excalibur, mais quand même. Peut etre prevoir un vaccin contre le tetanose)_ => Ajoute épée rouillée à l'inventaire
  - Échec : -aura _(elle ne bouge pas. Vous non plus d'ailleurs, vous vous êtes fait un tour de rein. aïe)_
- S'asseoir sur la souche => _"Vous méditez un instant. Un papillon se pose sur votre nez."_ +aura

**Renard**

- Observer => _"Un renard roux vous fixe avec une intelligence dérangeante."_
- Approcher doucement => _"Le renard s'approche, renifle votre main, et dépose une baie à vos pieds."_ => Ajoute baie mystérieuse à l'inventaire, +aura
- Courir après => -aura _(il est plus rapide, plus malin, et il le sait)_
- Parler au renard => _"Il penche la tête. Vous avez l'impression qu'il comprend. Il ne comprend pas."_

---

## 🌊 LE LAC

> _"Un lac d'un bleu étrangement parfait s'étale devant vous. La surface est lisse comme un miroir. Vous vous y voyez. Vous détournez le regard."_

**Directions :** Ouest => Plaine | Ouest-Nord => Forêt | Sud => Village

### Interactables directs du lac :

**Le Lac lui-même**

- Regarder son reflet => _"Vous voyez un paysan. Mais si vous plissez les yeux... non, c'est toujours un paysan."_
- Se baigner => +aura _(bain rafraîchissant, vous sentez moins le chou)_ + skip ticks
- Boire l'eau => _"L'eau est fraîche et pure. Vous vous sentez revigoré."_ +aura
- Jeter un objet dans le lac =>
  - Pièce : ++aura _(l'eau brille, une voix dit "Merci, ça faisait longtemps")_
  - Autre : -aura _(plouf. L'objet coule. Bravo.)_

**Vieille barque**

- Examiner => _"Une barque avec un trou. Classique."_
- Monter dedans _(sans réparation)_ => -aura _(vous coulez lentement en gardant votre dignité. Lentement.)_
- Réparer _(si planche dans inventaire)_ => Barque réparée, _"C'est pas joli, mais ça flotte."_ => Permet d'accéder à l'Île
- Monter dedans _(réparée)_ => Accès au point d'intérêt Île

---

### Points d'intérêt du lac :

### 🏝️ Île au milieu du lac _(accessible si barque réparée)_

> _"Une minuscule île avec un unique arbre tordu et un coffre recouvert de mousse. On dirait la planque d'un pirate qui avait un très petit budget."_

**Interactables :**

**Coffre moussu**

- Examiner => _"Un coffre en bois. Le cadenas est rouillé. Y'a un trou de serrure et aussi un bon gros cadenas."_
- Forcer le cadenas => Jet de réussite
  - Succès : +aura => ouvre le coffre
  - Échec : -aura _(vos doigts pleurent)_
- Frapper avec un objet =>
  - Épée rouillée : Ouvre le coffre, _"L'épée se brise mais le cadenas aussi. Fair trade."_ => Supprime épée
  - Marmite : Ouvre le coffre, _"BONG. Le bruit résonne sur tout le lac."_ +aura => Supprime marmite
  - Autre : _"Ça fait 'toc'. Le coffre s'en fiche."_
- Contenu du coffre _(si ouvert)_ : Cape brodée => ajout inventaire, ++aura _(une vraie cape ! Vous ressemblez presque à quelqu'un d'important !)_

**Arbre tordu**

- Examiner => _"Cet arbre pousse en spirale. La nature est bizarre."_
- Grimper => +aura _(la vue est magnifique, vous voyez le château du village !)_
- Secouer => _"Une noix de coco tombe. Vous n'êtes même pas sous les tropiques. Ne cherchez pas."_ => Ajoute noix de coco à l'inventaire

### 🎣 Ponton de pêche

> _"Un ponton branlant avance sur le lac. Un seau vide, une canne à pêche cassée, et une odeur de poisson qui date d'un autre siècle."_

**Interactables :**

**Canne à pêche cassée**

- Examiner => _"Cassée en deux. Comme vos rêves. Mais les rêves, ça se répare."_
- Réparer _(si corde dans inventaire)_ => Canne réparée, +aura => Permet de pêcher
- Pêcher _(si canne réparée)_ => Jet aléatoire
  - Poisson : +aura, _"Un poisson ! Pas gros, mais c'est le vôtre."_ => Ajoute poisson à l'inventaire
  - Botte : _"Une botte. Taille 47. Inutile mais amusant."_ => Ajoute vieille botte à l'inventaire
  - Rien : _"Ça ne mord pas. Les poissons sont au courant de votre situation sociale."_

**Seau**

- Examiner => _"Un seau vide. Symbole de votre vie actuelle."_
- Prendre => Ajoute seau à l'inventaire
- Mettre sur la tête => -aura _(vous ne voyez plus rien et trébuchez du ponton)_

---

## 🏘️ LE VILLAGE

> _"Le village de Bourg-les-Navets s'anime devant vous. Trois maisons, une taverne, une forge, et un château qui essaie très fort d'être impressionnant. Des poules se promènent avec plus d'assurance que vous."_

**Directions :** Nord => Plaine | Nord-Est => Le Lac

### Interactables directs du village :

**Poules**

- Caresser => _"La poule accepte. C'est doux. Vous repensez à vos choix de vie."_
- Courir après => -aura _(tout le village vous regarde. Les poules sont plus rapides.)_
- Parler aux poules => _"Cot. Cot cot. Cot."_ _(pas d'effet, mais vous avez essayé)_

**Fontaine du village**

- Boire => _"L'eau est tiède et a un goût de calcaire. C'est la meilleure eau que vous ayez bue."_
- Jeter une pièce _(si pièce dans inventaire)_ => +aura _(vous faites un vœu. Le vœu c'est d'avoir de l'aura. Meta.)_
- Se laver => +aura _(vous sentez moins le chou, les villageois vous regardent avec moins de dégoût)_
  Le Marchand (PNJ)

Acheter le "Philtre de Charisme Absolu" => Coûte la Pièce.

Si bu : -aura, -1 tick (C'était de l'eau du lac mélangée à du jus de chou. Vous êtes malade et perdez du temps).

Échanger des objets =>

## Rubis rutilant (du cimetière) => Échange contre une Armure rutilante (Donne un énorme boost d'Aura).

### Points d'intérêt du village :

### 🍺 Taverne "Au CochonPendu Pendu"

> _"La taverne sent la bière renversée et les décisions regrettables. Un barde chante faux dans un coin. Le tavernier essuie un verre qui n'a jamais été propre."_

**Interactables :**

**Le Tavernier (PNJ)**

- Parler => _"Bienvenue au Cochon Pendu ! On sert de la bière, des rumeurs, et des mauvais conseils."_
- Demander des nouvelles du roi => _"Le roi ? Il est de mauvaise humeur depuis que son bouffon a démissionné. Paraît qu'il cherche quelqu'un pour le remplacer..."_ => Indice pour le château
- Offrir un objet =>
  - Poisson : +aura, _"Un poisson frais ! Ça change du ragoût éternel. Tiens, bois un coup."_ => +aura bonus
  - Noix de coco : +aura, _"C'est quoi ce truc ? ...On va en faire un cocktail."_

**Le Barde (PNJ)**

- Écouter chanter => -aura _(c'est vraiment très mauvais)_
- Demander une chanson sur vous => Jet de réussite
  - Succès : ++aura _(la chanson est atroce mais tout le monde la retient, vous devenez célèbre !)_
  - Échec : -aura _("Le paysan qui pue le chou..." non merci.)_
- Offrir un objet =>
  - Œuf doré : ++aura, _"PAR LES DIEUX ! Un œuf de phoenix ! Je compose un OPÉRA en votre honneur !"_ => Renommée au village
  - Vieille botte : +aura, _"...je peux en faire un instrument. Ne demandez pas."_

**Tonneau dans le coin**

- Examiner => _"Un tonneau entrouvert. Ça sent fort."_
- Boire dedans => Jet aléatoire
  - Bon : +aura _(breuvage vigoureux !)_
  - Mauvais : -aura _(vinaigre. Ancien vinaigre.)_ + skip ticks
- Se cacher dedans => _"Vous vous cachez. Personne ne vous cherchait même pas (inserer blague)."_

---

### ⚒️ La Forge

> _"La chaleur vous frappe comme une gifle. Le forgeron, un colosse tatoué, tape sur une enclume avec la passion de quelqu'un qui règle des comptes avec le métal."_

**Interactables :**

**Le Forgeron (PNJ)**

- Parler => _"J'forge. Tu veux quoi ?"_
- Demander une armure => _"T'as de quoi payer ? Non ? Alors dégage. ...Ou ramène-moi quelque chose d'utile."_
- Offrir un objet =>
  - Épée rouillée : ++aura, _"Oh ! Du bon acier sous la rouille !"_ => Reforge en épée correcte (échange), ++aura
  - Médaille de l'ermite : +aura, _"Hmm, du bon métal. Tiens, je t'ai fait un bouclier."_ => Ajoute bouclier à l'inventaire
  - Farine enchantée : _"...c'est de la farine. J'suis forgeron."_ => Pas d'effet
- Demander à utiliser la forge => Jet de réussite
  - Succès : +aura _(vous forgez un truc. Pas sûr de ce que c'est mais c'est en métal.)_ => Ajoute bidule en métal à l'inventaire
  - Échec : -aura _(vous vous brûlez. Le forgeron soupire.)_

**Enclume**

- Examiner => _"Lourde. Très lourde. Inamovible."_
- Essayer de soulever => Jet de réussite
  - Succès : ++aura _(LE VILLAGE ENTIER VOUS ACCLAME)_ => Renommée
  - Échec : _"Non, votre dos refuse."_ -aura
- Taper dessus avec un objet =>
  - Marmite : _"BONG. Le forgeron est furieux. La marmite est cabossée."_ -aura

---

### 🏰 Le Château du Roi _(Zone finale)_

> _"Le château se dresse devant vous, majestueux et... légèrement de travers ? Le pont-levis grince. Les gardes ont l'air de s'ennuyer profondément."_

**Interactables :**

**Gardes (PNJ)**

- Parler => _"Halte. Qui va là. On dit ça parce qu'on doit, on sait très bien que c'est un paysan."_
- Demander à voir le roi => _"Le roi ne reçoit que les gens importants. T'es important ? ...T'as une cape au moins ?"_
  - Si cape brodée : _"Oh, jolie cape. Bon, entre."_ => Accès salle du trône
  - Si pas de cape : _"Reviens quand t'auras l'air de quelqu'un."_
- Corrompre _(si pièce ou objet de valeur)_ => +/- aura selon objet, ouvre l'accès
- Forcer le passage => -aura _(les gardes vous maîtrisent en 0.3 secondes sauf si epée)_

**Pont-levis**

- Examiner => _"Il a connu des jours meilleurs. Et des chevaliers meilleurs."_
- Traverser _(si accès autorisé)_ => Salle du trône

**Le Roi (PNJ)** _(salle du trône)_

- Se présenter => **ÉVALUATION FINALE DE L'AURA**
  - Aura suffisante : 🎉 _"Le roi vous regarde... sourit... et vous adoube ! SIR [JOUEUR] ! La foule applaudit ! Les poules applaudissent ! Même l'épouvantail applaudit probablement !"_ => **VICTOIRE**
  - Aura insuffisante : Jet de chance
    - Succès : _"Le roi hésite... mais votre culot lui plaît."_ => Adoubement de justesse => **VICTOIRE**
    - Échec : _"Le roi vous regarde, éclate de rire, et vous fait escorter dehors. Les poules se moquent."_ => **GAME OVER**
- Raconter ses aventures => Bonus d'aura basé sur objets et actions accomplis _(la broche, la cape, l'épée, les exploits mémorables ajoutent de l'aura narrative)_
- Offrir un objet au roi =>
  - Œuf doré : ++aura _(le roi ADORE)_
  - Talisman : +aura
  - Bidule en métal : _"C'est... quoi ? ...J'adore !"_ +aura
  - Marmite : _"...Gardes ?"_ -aura
  - Seau : --aura

---
