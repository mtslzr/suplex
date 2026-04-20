import { useQuery } from "@apollo/client/react";
import {
  TITLES_QUERY,
  type Promotion,
  type Title,
  type TitlesQuery,
} from "../graphql/operations";

function daysBetween(iso: string | null): number | null {
  if (!iso) return null;
  const then = new Date(iso).getTime();
  if (!Number.isFinite(then)) return null;
  const diff = Date.now() - then;
  return Math.max(0, Math.floor(diff / (1000 * 60 * 60 * 24)));
}

function formatDate(iso: string | null): string {
  if (!iso) return "—";
  const d = new Date(iso);
  return Number.isFinite(d.getTime()) ? d.toLocaleDateString() : "—";
}

function ChampionshipsPage() {
  const { data, loading, error } = useQuery<TitlesQuery>(TITLES_QUERY);

  const byPromotion = new Map<string, { promotion: Promotion; titles: Title[] }>();
  if (data) {
    for (const p of data.promotions) {
      byPromotion.set(p.id, { promotion: p, titles: [] });
    }
    for (const t of data.titles) {
      const bucket = byPromotion.get(t.promotionId);
      if (bucket) bucket.titles.push(t);
    }
  }

  const groups = Array.from(byPromotion.values())
    .filter((g) => g.titles.length > 0)
    .sort((a, b) => a.promotion.nickname.localeCompare(b.promotion.nickname));

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Championships</h1>
        <p className="mt-2 text-sm text-gray-500">
          Current champions per promotion. Sync promotions in Settings to
          populate this page.
        </p>
      </div>

      {loading && <p className="text-sm text-gray-500">Loading…</p>}
      {error && (
        <p className="text-sm text-red-600">
          Failed to load: {error.message}
        </p>
      )}
      {data && groups.length === 0 && (
        <p className="text-sm text-gray-500">
          No titles yet. Add promotions in Settings and hit Sync.
        </p>
      )}

      {groups.map(({ promotion, titles }) => (
        <section
          key={promotion.id}
          className="rounded-lg border border-gray-200 bg-white p-5"
          style={
            promotion.accentColor
              ? { borderLeftColor: promotion.accentColor, borderLeftWidth: 4 }
              : undefined
          }
        >
          <h2 className="text-lg font-semibold text-gray-900">
            {promotion.canonicalName ?? promotion.nickname}
            {promotion.abbreviation && (
              <span className="ml-2 text-sm font-normal text-gray-500">
                ({promotion.abbreviation})
              </span>
            )}
          </h2>
          <ul className="mt-3 divide-y divide-gray-200 border-t border-gray-200">
            {titles.map((t) => {
              const days = daysBetween(t.currentSinceDate);
              return (
                <li key={t.id} className="py-3">
                  <div className="flex flex-wrap items-baseline justify-between gap-2">
                    <div className="font-medium text-gray-900">{t.name}</div>
                    {t.cagematchUrl && (
                      <a
                        href={t.cagematchUrl}
                        target="_blank"
                        rel="noreferrer"
                        className="text-xs text-gray-500 underline hover:text-gray-700"
                      >
                        cagematch title page
                      </a>
                    )}
                  </div>
                  <div className="mt-1 text-sm text-gray-700">
                    {t.currentChampionDisplay ? (
                      <>
                        <span className="font-medium">
                          {t.currentChampionDisplay}
                        </span>
                        {t.currentSinceDate && (
                          <>
                            {" · since "}
                            {formatDate(t.currentSinceDate)}
                            {days !== null && (
                              <span className="text-gray-500">
                                {" "}
                                ({days} {days === 1 ? "day" : "days"})
                              </span>
                            )}
                          </>
                        )}
                      </>
                    ) : (
                      <span className="text-gray-500">Vacant</span>
                    )}
                  </div>
                </li>
              );
            })}
          </ul>
        </section>
      ))}
    </div>
  );
}

export default ChampionshipsPage;
