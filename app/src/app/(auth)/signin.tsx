import { Feature } from "@/components/Feature";
import { Button } from "@/components/ui/Button";
import {
  makeRedirectUri,
  useAuthRequest,
  useAutoDiscovery,
} from "expo-auth-session";
import { Link } from "expo-router";
import * as WebBrowser from "expo-web-browser";
import {
  ClockArrowUp,
  FolderKey,
  MonitorSmartphone,
  Palette,
  Settings,
  Tag,
} from "lucide-react-native";
import { useEffect } from "react";
import { Platform, Text, View } from "react-native";

WebBrowser.maybeCompleteAuthSession();

export default function SigninPage() {
  useEffect(() => {
    if (Platform.OS === "android") {
      WebBrowser.warmUpAsync();

      return () => {
        WebBrowser.coolDownAsync();
      };
    }
  }, []);

  const discovery = useAutoDiscovery(process.env.EXPO_PUBLIC_AUTH_SERVER!);
  const [request, result, promptAsync] = useAuthRequest(
    {
      clientId: process.env.EXPO_PUBLIC_AUTH_CLIENT_ID!,
      redirectUri: makeRedirectUri({
        scheme: "iceblink",
      }),
      scopes: ["openid", "profile"],
    },
    discovery
  );

  return (
    <>
      <View className="flex flex-col justify-center items-center gap-6 flex-1 self-stretch">
        <View className="flex flex-col items-center">
          <Text className="text-white text-3xl font-bold">
            Your all-in-one 2FA app
          </Text>
          <Text className="text-white text-2xl">Modern & bundled with:</Text>
        </View>

        <View className="bg-iceblink-bg-dim p-5 rounded-lg w-full">
          <Feature icon={FolderKey}>Encrypted cloud sync</Feature>
          <Feature icon={Palette}>Colors & icons for apps</Feature>
          <Feature icon={Tag}>Folders & tagging</Feature>
          <Feature icon={ClockArrowUp}>Ahead-of-time codes</Feature>
          <Feature icon={MonitorSmartphone}>Multiple device support</Feature>
        </View>
      </View>

      <View className="flex gap-3 w-full">
        <View className="flex flex-row gap-3">
          <Button
            color="primary"
            disabled={!request}
            onPress={() => promptAsync()}
            className="flex-grow"
          >
            Login
          </Button>
          <Link href="/(auth)/server" asChild>
            <Button color="secondary">
              <Settings />
            </Button>
          </Link>
        </View>
        <Link
          className="color-iceblink-fg-info text-lg text-right"
          href="/(auth)/unlock"
        >
          Continue offline
        </Link>
        {result && <Text>{JSON.stringify(result, null, 2)}</Text>}
      </View>
    </>
  );
}
