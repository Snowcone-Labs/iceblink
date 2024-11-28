import {
  makeRedirectUri,
  useAuthRequest,
  useAutoDiscovery,
} from "expo-auth-session";
import { Button, Text, View } from "react-native";

export default function SigninPage() {
  const discovery = useAutoDiscovery("https://pfapi.snowflake.blue");

  // Create and load an auth request
  const [request, result, promptAsync] = useAuthRequest(
    {
      clientId: "cm3c1gn37000c22v77my7iwnv",
      redirectUri: makeRedirectUri({
        scheme: "iceblink",
      }),
      scopes: ["openid", "profile"],
    },
    discovery
  );

  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
      className="bg-iceblink-bg-dark color-iceblink-fg-dark"
    >
      <Text className="text-5xl">Iceblink</Text>
      <Button title="Login" disabled={!request} onPress={() => promptAsync()} />
      {result && <Text>{JSON.stringify(result, null, 2)}</Text>}
    </View>
  );
}
