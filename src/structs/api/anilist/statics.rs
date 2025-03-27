pub static ANILIST_EMBED_COLOR: &str = "#00aaff";
pub static ANILIST_EMBED_AUTHOR_URL: &str = "https://anilist.co";
pub static ANILIST_EMBED_AUTHOR_ICON_URL: &str = "https://i.ibb.co/vYBvP34/anilist.png";
pub static ANILIST_ANIME_FIELDS: &str = "
	id siteUrl coverImage { extraLarge } bannerImage
	countryOfOrigin title { romaji native english } format synonyms isAdult
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
pub static ANILIST_MANGA_FIELDS: &str = "
	id siteUrl coverImage { extraLarge } bannerImage
	countryOfOrigin title { romaji native english } format synonyms isAdult
	startDate { year month day }
	endDate { year month day }
	status
	chapters volumes isLicensed
	genres source averageScore meanScore
	externalLinks { site url }
	rankings { rank type format allTime season year }
	popularity favourites
	description
	characters(perPage: 10, sort: ROLE) {
		edges {
			node { name { full } image { large } siteUrl }
			role
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
pub static ANILIST_USER_FIELDS: &str = "
	id siteUrl avatar { large } name createdAt updatedAt about
	statistics {
		anime { episodesWatched minutesWatched meanScore }
		manga { chaptersRead volumesRead meanScore }
	}
	favourites {
		anime {
			nodes {
				title { romaji }
				format siteUrl
			}
		}
		manga {
			nodes {
				title { romaji }
				format siteUrl
			}
		}
		characters {
			nodes {
				name { full } image { large } siteUrl
			}
		}
		staff {
			nodes {
				name { full } image { large } siteUrl
			}
		}
	}
";
