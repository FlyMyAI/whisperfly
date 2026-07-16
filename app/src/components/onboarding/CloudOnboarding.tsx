import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import { commands } from "@/bindings";
import HandyTextLogo from "../icons/HandyTextLogo";
import { Input } from "../ui/Input";
import { useSettings } from "../../hooks/useSettings";

const PUBLIC_AGENT_URL = "https://app.flymy.ai/agents/chat/kff-gefa-yjr";
const FLYMYAI_URL = "https://app.flymy.ai/";

interface CloudOnboardingProps {
  onComplete: () => void;
}

/**
 * First-run wizard: connect the user's OWN FlyMy.AI account. No credentials
 * ship with the app - the user brings their key and clones the public agent
 * to their account. Accepts the chat-link id and resolves it to the agent
 * uuid automatically (everyone pastes the chat id - we did too).
 */
const CloudOnboarding: React.FC<CloudOnboardingProps> = ({ onComplete }) => {
  const { t } = useTranslation();
  const { updateSetting } = useSettings();
  const [apiKey, setApiKey] = useState("");
  const [agentRef, setAgentRef] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const keyLooksValid = apiKey.trim().startsWith("fly-");

  const handleFinish = async () => {
    setError(null);
    if (!keyLooksValid) {
      setError(t("cloudOnboarding.errors.badKey"));
      return;
    }
    if (!agentRef.trim()) {
      setError(t("cloudOnboarding.errors.noAgent"));
      return;
    }
    setBusy(true);
    try {
      const resolved = await commands.resolveFlymyaiAgent(
        agentRef.trim(),
        apiKey.trim(),
      );
      if (resolved.status !== "ok") {
        setError(resolved.error);
        setBusy(false);
        return;
      }
      await updateSetting("flymyai_api_key", apiKey.trim());
      await updateSetting("flymyai_agent_uuid", resolved.data);
      await updateSetting("whisperfly_cloud_enabled", true);
      onComplete();
    } catch (e) {
      setError(String(e));
      setBusy(false);
    }
  };

  const stepClass =
    "flex flex-col gap-2 rounded-lg border border-mid-gray/25 p-4 text-left";
  const buttonClass =
    "px-3 py-1.5 text-sm rounded-md border border-mid-gray/40 hover:bg-mid-gray/10 transition-colors self-start";

  return (
    <div className="h-screen w-screen flex flex-col items-center justify-center p-6 gap-6 overflow-y-auto">
      <div className="flex flex-col items-center gap-2 shrink-0">
        <HandyTextLogo width={220} />
        <p className="text-text/70 max-w-md font-medium text-center">
          {t("cloudOnboarding.subtitle")}
        </p>
      </div>

      <div className="max-w-[560px] w-full flex flex-col gap-4">
        <div className={stepClass}>
          <h2 className="font-semibold">{t("cloudOnboarding.step1.title")}</h2>
          <p className="text-sm text-text/60">
            {t("cloudOnboarding.step1.description")}
          </p>
          <button
            type="button"
            className={buttonClass}
            onClick={() => openUrl(PUBLIC_AGENT_URL)}
          >
            {t("cloudOnboarding.step1.button")}
          </button>
          <Input
            type="text"
            value={agentRef}
            onChange={(e) => setAgentRef(e.target.value)}
            placeholder={t("cloudOnboarding.step1.placeholder")}
            variant="compact"
            className="font-mono"
          />
        </div>

        <div className={stepClass}>
          <h2 className="font-semibold">{t("cloudOnboarding.step2.title")}</h2>
          <p className="text-sm text-text/60">
            {t("cloudOnboarding.step2.description")}
          </p>
          <button
            type="button"
            className={buttonClass}
            onClick={() => openUrl(FLYMYAI_URL)}
          >
            {t("cloudOnboarding.step2.button")}
          </button>
          <Input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder={t("cloudOnboarding.step2.placeholder")}
            variant="compact"
            className={apiKey && !keyLooksValid ? "border-red-500" : ""}
          />
          {apiKey && !keyLooksValid && (
            <p className="text-xs text-red-500">
              {t("cloudOnboarding.errors.badKey")}
            </p>
          )}
        </div>

        {error && <p className="text-sm text-red-500 text-center">{error}</p>}

        <div className="flex items-center justify-between">
          <button
            type="button"
            className="text-sm text-text/50 hover:text-text transition-colors"
            onClick={onComplete}
          >
            {t("cloudOnboarding.skip")}
          </button>
          <button
            type="button"
            disabled={busy}
            className="px-4 py-2 rounded-md bg-text text-background font-medium disabled:opacity-50"
            onClick={handleFinish}
          >
            {busy ? t("cloudOnboarding.checking") : t("cloudOnboarding.finish")}
          </button>
        </div>
      </div>
    </div>
  );
};

export default CloudOnboarding;
