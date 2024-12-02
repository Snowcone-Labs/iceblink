import { Button } from "@/components/ui/Button";
import Input from "@/components/ui/Input";
import Tip from "@/components/ui/Tip";
import { Link } from "expo-router";
import { useState } from "react";
import { Text, View } from "react-native";

export default function UnlockPage() {
  const [server, setServer] = useState("https://iceblink.snowflake.blue");

  return (
    <View className="bg-iceblink-bg-dark p-5 justify-center items-center flex-1 gap-6">
      <Text className="color-iceblink-fg-dark text-5xl">Set sync server</Text>

      <Input value={server} onChange={setServer} />

      {server !== "https://iceblink.snowflake.blue" && (
        <Tip
          content="You are not using the offical Iceblink Sync server. Make sure you trust this server!"
          type="warning"
        />
      )}

      <Link href="/(auth)/signin" asChild>
        <Button color="primary">Done</Button>
      </Link>
    </View>
  );
}
