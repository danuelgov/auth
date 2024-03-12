import { Body, Head, Html, Preview, Text } from "@react-email/components";
import { FONT_FAMILY } from "./shared/font";

export default function UserCreatedEmail(): JSX.Element {
  return (
    <Html>
      <Head />
      <Preview>🔑 새로운 계정이 생성 되었습니다.</Preview>
      <Body>
        <Text style={text}>
          다뉴엘 거버넌스에 새로운 계정({"{{EMAIL_ADDRESS}}"})이 생성
          되었습니다.
        </Text>
      </Body>
    </Html>
  );
}

const text = {
  fontFamily: FONT_FAMILY,
};
