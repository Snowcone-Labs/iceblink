import { IceblinkLogo } from "@/components/Logo";
import { Button } from "@/components/ui/Button";
import {
  makeRedirectUri,
  useAuthRequest,
  useAutoDiscovery,
} from "expo-auth-session";
import { Link } from "expo-router";
import * as WebBrowser from "expo-web-browser";
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
    <View className="bg-iceblink-bg-dark p-5 justify-center items-center flex-1">
      <View className="my-auto flex justify-center items-center">
        <Text className="color-iceblink-fg-dark text-5xl">Iceblink</Text>
        <IceblinkLogo size={200} />
      </View>
      <View className="flex gap-3">
        <Button
          color="success"
          disabled={!request}
          onPress={() => promptAsync()}
        >
          Login
        </Button>
        <Link className="color-iceblink-fg-info text-lg" href="/(auth)/unlock">
          Continue offline
        </Link>
      </View>
      {result && <Text>{JSON.stringify(result, null, 2)}</Text>}
    </View>
  );
}
