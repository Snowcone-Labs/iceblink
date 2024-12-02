import { Button } from "@/components/ui/Button";
import Input from "@/components/ui/Input";
import { Link } from "expo-router";
import { Text, View } from "react-native";

export default function UnlockPage() {
  return (
    <View className="bg-iceblink-bg-dark p-5 justify-center items-center flex-1 gap-6">
      <Text className="color-iceblink-fg-dark text-5xl">Set sync server</Text>

      <Input placeholder="https://iceblink.snowflake.blue" />

      <Link href="/(auth)/signin" asChild>
        <Button color="primary">Done</Button>
      </Link>
    </View>
  );
}
