import { IceblinkTextLogo } from "@/components/TextLogo";
import { Button } from "@/components/ui/Button";
import { randomInArray } from "@/utils";
import { Link } from "expo-router";
import { useState } from "react";
import { Text, TouchableOpacity, View } from "react-native";

export default function NotFoundScreen() {
  const cringeJokes = [
    "This page is colder than an expired 2FA code.",
    "We blinked and this page vanished into the ice. Maybe it's in another time window?",
    "Just like a 2FA code, this page has expired and melted away.",
    "Oops! This page must have blinked itself into the wrong time window.",
    "Looks like this link froze in time and stopped working.",
    "It's not found, just like that last 2FA code you typoed.",
    "No code, no page, just ice.",
    "Time's up — this page expired.",
    "No blink, no link, just 404.",
    "Climate change causes ice to melt.",
  ];
  const [headline, setHeadline] = useState(randomInArray(cringeJokes));

  return (
    <View className="flex flex-col items-center justify-between h-full p-4">
      <IceblinkTextLogo />

      <View>
        <Text className="text-center text-2xl font-bold text-iceblink-fg-dark">
          Page not found
        </Text>
        <TouchableOpacity
          onPress={() => setHeadline(randomInArray(cringeJokes))}
        >
          <Text className="text-center text-lg text-iceblink-fg-dim">
            {headline}
          </Text>
        </TouchableOpacity>
      </View>

      <Link href="/" asChild>
        <Button>Go home</Button>
      </Link>
    </View>
  );
}
