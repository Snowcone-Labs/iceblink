import { IceblinkTextLogo } from "@/components/TextLogo";
import Button from "@/components/ui/Button";
import { Link } from "expo-router";
import React from "react";
import { Text, View } from "react-native";

export default function NotFoundScreen() {
  return (
    <View className="flex flex-col items-center justify-between h-full p-4">
      <IceblinkTextLogo />

      <View>
        <Text className="text-center text-2xl text-iceblink-fg-dark">
          You look lost
        </Text>
        <Text className="text-center text-lg text-iceblink-fg-dim">
          This page was not found.
        </Text>
      </View>

      <Link href="/" asChild>
        <Button>Go home</Button>
      </Link>
    </View>
  );
}
