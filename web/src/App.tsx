import { Routes, Route } from "react-router";
import Layout from "./components/Layout";
import SchedulePage from "./pages/SchedulePage";
import PromotionsPage from "./pages/PromotionsPage";
import ChampionshipsPage from "./pages/ChampionshipsPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<SchedulePage />} />
        <Route path="/promotions" element={<PromotionsPage />} />
        <Route path="/championships" element={<ChampionshipsPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}

export default App;
