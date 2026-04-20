import { useState } from "react";
import { useMutation, useQuery } from "@apollo/client/react";
import {
  ADD_PROMOTION,
  PROMOTIONS_QUERY,
  REMOVE_PROMOTION,
  SCRAPE_ALL_PROMOTIONS,
  SCRAPE_PROMOTION,
  UPDATE_PROMOTION,
  type PromotionsQuery,
} from "../graphql/operations";

function SettingsPage() {
  const { data, loading, error, refetch } =
    useQuery<PromotionsQuery>(PROMOTIONS_QUERY);
  const [addPromotion, { loading: adding, error: addError }] =
    useMutation(ADD_PROMOTION);
  const [updatePromotion] = useMutation(UPDATE_PROMOTION);
  const [removePromotion] = useMutation(REMOVE_PROMOTION);
  const [scrapePromotion] = useMutation(SCRAPE_PROMOTION);
  const [scrapeAll, { loading: scrapingAll }] =
    useMutation(SCRAPE_ALL_PROMOTIONS);

  const [nickname, setNickname] = useState("");
  const [cagematchId, setCagematchId] = useState("");
  const [syncing, setSyncing] = useState<Record<string, boolean>>({});
  const [status, setStatus] = useState<string | null>(null);

  const handleAdd = async (event: React.FormEvent) => {
    event.preventDefault();
    const parsed = Number.parseInt(cagematchId, 10);
    if (!nickname.trim() || !Number.isFinite(parsed)) return;
    await addPromotion({
      variables: { nickname: nickname.trim(), cagematchId: parsed },
    });
    setNickname("");
    setCagematchId("");
    await refetch();
  };

  const handleToggle = async (id: string, enabled: boolean) => {
    await updatePromotion({ variables: { id, enabled: !enabled } });
    await refetch();
  };

  const handleRemove = async (id: string) => {
    await removePromotion({ variables: { id } });
    await refetch();
  };

  const handleSync = async (id: string) => {
    setSyncing((s) => ({ ...s, [id]: true }));
    setStatus(null);
    try {
      const result = await scrapePromotion({ variables: { id } });
      const payload = result.data as
        | {
            scrapePromotion?: {
              titlesCreated: number;
              titlesUpdated: number;
              error: string | null;
            };
          }
        | undefined;
      const r = payload?.scrapePromotion;
      if (r?.error) {
        setStatus(`Sync failed: ${r.error}`);
      } else if (r) {
        setStatus(
          `Synced · ${r.titlesCreated} titles added, ${r.titlesUpdated} updated`,
        );
      }
      await refetch();
    } finally {
      setSyncing((s) => ({ ...s, [id]: false }));
    }
  };

  const handleSyncAll = async () => {
    setStatus(null);
    const result = await scrapeAll();
    const payload = result.data as
      | {
          scrapeAllPromotions?: Array<{
            titlesCreated: number;
            titlesUpdated: number;
            error: string | null;
          }>;
        }
      | undefined;
    const rows = payload?.scrapeAllPromotions ?? [];
    const errors = rows.filter((r) => r.error).length;
    const created = rows.reduce((s, r) => s + r.titlesCreated, 0);
    const updated = rows.reduce((s, r) => s + r.titlesUpdated, 0);
    setStatus(
      `Synced ${rows.length} promotions · ${created} titles added, ${updated} updated${
        errors ? ` · ${errors} errors` : ""
      }`,
    );
    await refetch();
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-gray-900">Settings</h1>

      <section className="rounded-lg border border-gray-200 bg-white p-5">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h2 className="text-lg font-semibold text-gray-900">Promotions</h2>
            <p className="mt-1 text-sm text-gray-500">
              Add promotions by their cagematch.net ID (the <code>nr=</code>{" "}
              value in the URL). Sync pulls metadata, titles, and current
              champions.
            </p>
          </div>
          <button
            type="button"
            onClick={handleSyncAll}
            disabled={scrapingAll || !data?.promotions.length}
            className="shrink-0 rounded-md border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-50"
          >
            {scrapingAll ? "Syncing…" : "Sync all"}
          </button>
        </div>

        <form onSubmit={handleAdd} className="mt-4 flex flex-wrap items-end gap-3">
          <label className="flex flex-col text-sm text-gray-700">
            Nickname
            <input
              type="text"
              value={nickname}
              onChange={(e) => setNickname(e.target.value)}
              placeholder="AEW"
              required
              className="mt-1 rounded-md border border-gray-300 px-3 py-1.5 text-sm"
            />
          </label>
          <label className="flex flex-col text-sm text-gray-700">
            Cagematch ID
            <input
              type="number"
              value={cagematchId}
              onChange={(e) => setCagematchId(e.target.value)}
              placeholder="2287"
              required
              min={1}
              className="mt-1 rounded-md border border-gray-300 px-3 py-1.5 text-sm"
            />
          </label>
          <button
            type="submit"
            disabled={adding}
            className="rounded-md bg-gray-900 px-3 py-1.5 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50"
          >
            {adding ? "Validating…" : "Add"}
          </button>
        </form>
        {addError && (
          <p className="mt-2 text-sm text-red-600">{addError.message}</p>
        )}
        {status && <p className="mt-2 text-sm text-gray-700">{status}</p>}

        <div className="mt-6">
          {loading && <p className="text-sm text-gray-500">Loading…</p>}
          {error && (
            <p className="text-sm text-red-600">Failed to load: {error.message}</p>
          )}
          {data && data.promotions.length === 0 && (
            <p className="text-sm text-gray-500">
              No promotions tracked yet. Add one above.
            </p>
          )}
          {data && data.promotions.length > 0 && (
            <ul className="divide-y divide-gray-200 border-t border-gray-200">
              {data.promotions.map((p) => {
                const isSyncing = syncing[p.id];
                return (
                  <li
                    key={p.id}
                    className="flex items-center justify-between py-3"
                  >
                    <div>
                      <div className="font-medium text-gray-900">
                        {p.nickname}
                        {p.abbreviation && p.abbreviation !== p.nickname && (
                          <span className="ml-2 text-xs font-normal text-gray-500">
                            ({p.abbreviation})
                          </span>
                        )}
                      </div>
                      <div className="text-xs text-gray-500">
                        {p.canonicalName ?? "(not yet scraped)"} · cagematch{" "}
                        <a
                          href={p.cagematchUrl ?? "#"}
                          target="_blank"
                          rel="noreferrer"
                          className="underline hover:text-gray-700"
                        >
                          #{p.cagematchId}
                        </a>
                        {p.lastSyncedAt && (
                          <>
                            {" · last synced "}
                            {new Date(p.lastSyncedAt).toLocaleString()}
                          </>
                        )}
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      <label className="flex items-center gap-2 text-sm text-gray-700">
                        <input
                          type="checkbox"
                          checked={p.enabled}
                          onChange={() => handleToggle(p.id, p.enabled)}
                        />
                        Enabled
                      </label>
                      <button
                        type="button"
                        onClick={() => handleSync(p.id)}
                        disabled={isSyncing}
                        className="rounded-md border border-gray-300 px-2 py-1 text-xs text-gray-700 hover:bg-gray-50 disabled:opacity-50"
                      >
                        {isSyncing ? "Syncing…" : "Sync"}
                      </button>
                      <button
                        type="button"
                        onClick={() => handleRemove(p.id)}
                        className="rounded-md border border-gray-300 px-2 py-1 text-xs text-gray-700 hover:bg-gray-50"
                      >
                        Remove
                      </button>
                    </div>
                  </li>
                );
              })}
            </ul>
          )}
        </div>
      </section>
    </div>
  );
}

export default SettingsPage;
