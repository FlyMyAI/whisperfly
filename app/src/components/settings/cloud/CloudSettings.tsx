import React from "react";
import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { SettingContainer } from "../../ui/SettingContainer";
import { ToggleSwitch } from "../../ui/ToggleSwitch";
import { Input } from "../../ui/Input";
import { ApiKeyField } from "../PostProcessingSettingsApi/ApiKeyField";
import { useSettings } from "../../../hooks/useSettings";
import { commands } from "@/bindings";

/**
 * FlyMy.AI Cloud: ship every finished voice note to a FlyMyAI agent
 * (cloud STT + cleanup + keywords + Notion filing). Agents are per-account:
 * users clone the public WhisperFly agent and paste their copy's uuid.
 */
const PUBLIC_AGENT_URL = "https://app.flymy.ai/agents/chat/kff-gefa-yjr";

export const CloudSettings: React.FC = () => {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating } = useSettings();

  const enabled = getSetting("whisperfly_cloud_enabled") || false;

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("settings.cloud.title")}>
        <ToggleSwitch
          checked={enabled}
          onChange={(value) => updateSetting("whisperfly_cloud_enabled", value)}
          isUpdating={isUpdating("whisperfly_cloud_enabled")}
          label={t("settings.cloud.enabled.label")}
          description={t("settings.cloud.enabled.description")}
          grouped={true}
        />
        <SettingContainer
          title={t("settings.cloud.apiKey.title")}
          description={t("settings.cloud.apiKey.description")}
          layout="horizontal"
          grouped={true}
        >
          <ApiKeyField
            value={getSetting("flymyai_api_key") || ""}
            onBlur={(value) => updateSetting("flymyai_api_key", value)}
            disabled={false}
            placeholder={t("settings.cloud.apiKey.placeholder")}
          />
        </SettingContainer>
        <SettingContainer
          title={t("settings.cloud.getAgent.title")}
          description={t("settings.cloud.getAgent.description")}
          layout="horizontal"
          grouped={true}
        >
          <button
            type="button"
            className="px-3 py-1.5 text-sm rounded-md border border-mid-gray/40 hover:bg-mid-gray/10 transition-colors"
            onClick={() => openUrl(PUBLIC_AGENT_URL)}
          >
            {t("settings.cloud.getAgent.button")}
          </button>
        </SettingContainer>
        <SettingContainer
          title={t("settings.cloud.agentUuid.title")}
          description={t("settings.cloud.agentUuid.description")}
          layout="horizontal"
          grouped={true}
        >
          <AgentUuidField
            value={getSetting("flymyai_agent_uuid") || ""}
            onBlur={async (value) => {
              const v = value.trim();
              if (!v) return;
              // Accept the chat-link id too and resolve it to the agent uuid.
              const resolved = await commands.resolveFlymyaiAgent(
                v,
                getSetting("flymyai_api_key") || "",
              );
              updateSetting(
                "flymyai_agent_uuid",
                resolved.status === "ok" ? resolved.data : v,
              );
            }}
            placeholder={t("settings.cloud.agentUuid.placeholder")}
          />
        </SettingContainer>
        <SettingContainer
          title={t("settings.cloud.connect.title")}
          description={t("settings.cloud.connect.description")}
          layout="horizontal"
          grouped={true}
        >
          <div className="flex gap-2">
            <button
              type="button"
              className="px-3 py-1.5 text-sm rounded-md border border-mid-gray/40 hover:bg-mid-gray/10 transition-colors"
              onClick={() => openUrl("https://app.flymy.ai/mcp-configs#notion")}
            >
              {t("settings.cloud.connect.notionButton")}
            </button>
            <button
              type="button"
              className="px-3 py-1.5 text-sm rounded-md border border-mid-gray/40 hover:bg-mid-gray/10 transition-colors"
              onClick={() => openUrl("https://app.flymy.ai/")}
            >
              {t("settings.cloud.connect.keyButton")}
            </button>
          </div>
        </SettingContainer>
      </SettingsGroup>
    </div>
  );
};

interface AgentUuidFieldProps {
  value: string;
  onBlur: (value: string) => void;
  placeholder?: string;
}

const AgentUuidField: React.FC<AgentUuidFieldProps> = React.memo(
  ({ value, onBlur, placeholder }) => {
    const [localValue, setLocalValue] = React.useState(value);

    React.useEffect(() => {
      setLocalValue(value);
    }, [value]);

    return (
      <Input
        type="text"
        value={localValue}
        onChange={(event) => setLocalValue(event.target.value)}
        onBlur={() => onBlur(localValue)}
        placeholder={placeholder}
        variant="compact"
        className="flex-1 min-w-[320px] font-mono"
      />
    );
  },
);

AgentUuidField.displayName = "AgentUuidField";
