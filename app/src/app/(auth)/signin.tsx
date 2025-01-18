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
  Cloud,
  CloudOff,
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

        <View className="bg-iceblink-bg-dim p-5 rounded-lg w-full gap-4">
          <Feature icon={FolderKey}>Encrypted cloud sync</Feature>
          <Feature icon={Palette}>Colors & icons for apps</Feature>
          <Feature icon={Tag}>Folders & tagging</Feature>
          <Feature icon={ClockArrowUp}>Ahead-of-time codes</Feature>
          <Feature icon={MonitorSmartphone}>Multiple device support</Feature>
        </View>
      </View>

      <View className="flex flex-col gap-5 w-full">
        <View className="flex flex-row gap-5">
          <Link href="/(auth)/server" asChild>
            <Button color="secondary" icon={Settings} />
          </Link>

          <Button
            color="primary"
            disabled={!request}
            onPress={() => promptAsync()}
            className="flex-grow"
            icon={Cloud}
          >
            Login to sync
          </Button>
        </View>

        <Link href="/(auth)/unlock" asChild>
          <Button color="secondary" icon={CloudOff}>
            Start locally
          </Button>
        </Link>

        {result && <Text>{JSON.stringify(result, null, 2)}</Text>}
      </View>
    </>
  );
}
