pub static ANILIST_ANIME_FIELDS: &str = "
	coverImage { extraLarge } bannerImage
	countryOfOrigin title { romaji native english } format synonyms siteUrl isAdult
	startDate { year month day }
	endDate { year month day }
	status
	airingSchedule { nodes { timeUntilAiring } }
	season seasonYear trailer { id site }
	episodes duration hashtag
	genres source averageScore meanScore
	externalLinks { site url }
	rankings { rank type format allTime season year }
	popularity favourites
	description
	studios { nodes { name siteUrl } }
	characters(perPage: 10, sort: ROLE) {
		edges {
			node { name { full } image { large } siteUrl }
			role
			voiceActors { name { full } languageV2 siteUrl }
		}
	}
	relations {
		edges {
			relationType
			node { title { romaji native english } format siteUrl }
		}
	}
	updatedAt
";
