import { gql } from "@apollo/client";

export type Promotion = {
  id: string;
  cagematchId: number;
  nickname: string;
  canonicalName: string | null;
  country: string | null;
  logoUrl: string | null;
  cagematchUrl: string | null;
  accentColor: string | null;
  enabled: boolean;
  lastSyncedAt: string | null;
};

export type PromotionsQuery = {
  promotions: Promotion[];
};

export const PROMOTIONS_QUERY = gql`
  query Promotions {
    promotions {
      id
      cagematchId
      nickname
      canonicalName
      country
      logoUrl
      cagematchUrl
      accentColor
      enabled
      lastSyncedAt
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
