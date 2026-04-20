import { gql } from "@apollo/client";

export type Promotion = {
  id: string;
  cagematchId: number;
  nickname: string;
  canonicalName: string | null;
  abbreviation: string | null;
  country: string | null;
  logoUrl: string | null;
  cagematchUrl: string | null;
  accentColor: string | null;
  enabled: boolean;
  lastSyncedAt: string | null;
};

export type Title = {
  id: string;
  promotionId: string;
  cagematchId: number;
  name: string;
  isActive: boolean;
  cagematchUrl: string | null;
  currentChampionDisplay: string | null;
  currentChampionCagematchId: number | null;
  currentSinceDate: string | null;
  lastSyncedAt: string | null;
};

export type PromotionsQuery = {
  promotions: Promotion[];
};

export type TitlesQuery = {
  titles: Title[];
  promotions: Promotion[];
};

export const PROMOTIONS_QUERY = gql`
  query Promotions {
    promotions {
      id
      cagematchId
      nickname
      canonicalName
      abbreviation
      country
      logoUrl
      cagematchUrl
      accentColor
      enabled
      lastSyncedAt
    }
  }
`;

export const TITLES_QUERY = gql`
  query Titles {
    titles(activeOnly: true) {
      id
      promotionId
      cagematchId
      name
      isActive
      cagematchUrl
      currentChampionDisplay
      currentChampionCagematchId
      currentSinceDate
      lastSyncedAt
    }
    promotions {
      id
      nickname
      canonicalName
      abbreviation
      accentColor
    }
  }
`;

export const ADD_PROMOTION = gql`
  mutation AddPromotion($nickname: String!, $cagematchId: Int!) {
    addPromotion(input: { nickname: $nickname, cagematchId: $cagematchId }) {
      id
    }
  }
`;

export const UPDATE_PROMOTION = gql`
  mutation UpdatePromotion($id: UUID!, $enabled: Boolean, $nickname: String) {
    updatePromotion(id: $id, input: { enabled: $enabled, nickname: $nickname }) {
      id
      enabled
      nickname
    }
  }
`;

export const REMOVE_PROMOTION = gql`
  mutation RemovePromotion($id: UUID!) {
    removePromotion(id: $id)
  }
`;

export const SCRAPE_PROMOTION = gql`
  mutation ScrapePromotion($id: UUID!) {
    scrapePromotion(id: $id) {
      promotionId
      titlesCreated
      titlesUpdated
      error
    }
  }
`;

export const SCRAPE_ALL_PROMOTIONS = gql`
  mutation ScrapeAllPromotions {
    scrapeAllPromotions {
      promotionId
      titlesCreated
      titlesUpdated
      error
    }
  }
`;
