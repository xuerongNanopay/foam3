package foam.nanos.auth.oidc;

import foam.core.X;
import foam.nanos.http.WebAgent;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.OutputStream;
import javax.net.ssl.HttpsURLConnection;
import java.net.URL;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;

import org.apache.commons.io.IOUtils;
import foam.nanos.logger.Logger;
import foam.nanos.google.api.auth.GoogleApiCredentials;

public class OIDCWebAgent implements WebAgent {
    private static final String TOKEN_ENDPOINT = "https://oauth2.googleapis.com/token";
    private static final String REDIRECT_URI = "http://localhost:8080/service/oidc"; // Replace with your redirect URI

    private GoogleApiCredentials getCredentials(foam.core.X x) {
        return (GoogleApiCredentials)(((foam.dao.DAO)x.get("googleApiCredentialsDAO")).find("localhost:8080"));
    }

    @Override
    public void execute(X x) {
        Logger logger = (Logger) x.get("logger");
        HttpServletRequest req = x.get(HttpServletRequest.class);
        HttpServletResponse resp = x.get(HttpServletResponse.class);

        try {
            String code = req.getParameter("code");
            if (code == null || code.isEmpty()) {
                resp.setStatus(HttpServletResponse.SC_BAD_REQUEST);
                resp.getWriter().write("Missing authorization code");
                return;
            }

            // Exchange authorization code for tokens
            String token = getTokenFromAuthCode(x, code);
            if (token == null) {
                resp.setStatus(HttpServletResponse.SC_INTERNAL_SERVER_ERROR);
                resp.getWriter().write("Failed to obtain tokens");
                return;
            }

            javax.json.JsonReader reader;

            reader = javax.json.Json.createReader(new java.io.StringReader(token));
            javax.json.JsonObject tokenResponse = reader.readObject();
            reader.close();

            // Process the ID token as required
            // For example, you can parse the JWT and verify its claims

            foam.nanos.auth.AuthenticationService authn = (foam.nanos.auth.AuthenticationService)x.get("authentication");

            String parts[] = tokenResponse.getString("id_token").split("\\.");
            String headerb64 = parts[0];
            String bodyb64 = parts[1];
            String signatureb64 = parts[2];

            //byte[] headerBytes = java.util.Base64.getUrlDecoder().decode(headerb64);
            byte[] bodyBytes = java.util.Base64.getUrlDecoder().decode(bodyb64);
            //byte[] signatureBytes = java.util.Base64.getUrlDecoder().decode(signatureb64);

            //String header = new String(headerBytes, java.nio.charset.StandardCharsets.UTF_8);
            String body = new String(bodyBytes, java.nio.charset.StandardCharsets.UTF_8);

            //reader = javax.json.Json.createReader(new java.io.StringReader(header));
            //javax.json.JsonObject headerObject = reader.readObject();
            //reader.close();

            reader = javax.json.Json.createReader(new java.io.StringReader(body));
            javax.json.JsonObject bodyObject = reader.readObject();
            reader.close();

            if (!bodyObject.getBoolean("email_verified")) {
                throw new foam.nanos.auth.AuthenticationException("email is not verified");
            }

            if (bodyObject.getInt("exp", Integer.MAX_VALUE) < java.time.Instant.now().getEpochSecond()) {
                throw new foam.nanos.auth.AuthenticationException("expired token");
            }

            String expectedAudience = getCredentials(x).getClientId();

            if (!bodyObject.getString("aud").equals(expectedAudience)) {
                throw new foam.nanos.auth.AuthenticationException("incorrect audience");
            }

            String email = bodyObject.getString("email");

            foam.nanos.auth.User user = ((foam.nanos.auth.UniqueUserService)x.get("uniqueUserService")).getUser(x, email);

            if ( user == null ) {
                throw new foam.nanos.auth.UserNotFoundException();
            }

            String sessionID = req.getParameter("state");
            foam.nanos.session.Session session = (foam.nanos.session.Session)((foam.dao.DAO)x.get("sessionDAO")).find(sessionID);
            if ( session == null ) {
                throw new RuntimeException("session not found");
            }

            authn.login(session.getContext(), user);

            resp.setStatus(HttpServletResponse.SC_OK);
            resp.setContentType("text/html");
            resp.getWriter().write("<!DOCTYPE html><html><body><h1>Login Success</h1><script language=\"javascript\">window.opener.postMessage({ msg: \"success\", sessionID: \"" + sessionID + "\" }, location.origin);</script></body></html>");
        } catch (Exception e) {
            e.printStackTrace();
            try {
                resp.setStatus(HttpServletResponse.SC_INTERNAL_SERVER_ERROR);
                resp.getWriter().write("Server error");
            } catch (Exception ex) {
                ex.printStackTrace();
            }
        }
    }

    private String getTokenFromAuthCode(X x, String code) {
        Logger logger = (Logger) x.get("logger");
        try {
            URL url = new URL(TOKEN_ENDPOINT);
            HttpsURLConnection conn = (HttpsURLConnection) url.openConnection();
            conn.setRequestMethod("POST");
            conn.setDoOutput(true);
            conn.setRequestProperty("Content-Type", "application/x-www-form-urlencoded");

            GoogleApiCredentials creds = getCredentials(x);
            logger.info("creds", creds);

            String params = "code=" + URLEncoder.encode(code, "UTF-8") +
                    "&client_id=" + URLEncoder.encode(creds.getClientId(), "UTF-8") +
                    "&client_secret=" + URLEncoder.encode(creds.getClientSecret(), "UTF-8") +
                    "&redirect_uri=" + URLEncoder.encode(REDIRECT_URI, "UTF-8") +
                    "&grant_type=authorization_code";

            try (OutputStream os = conn.getOutputStream()) {
                os.write(params.getBytes(StandardCharsets.UTF_8));
            }

            if (conn.getResponseCode() != 200) {
                logger.error("Failed to obtain tokens, HTTP response code: " + conn.getResponseCode());
                return null;
            }

            try (BufferedReader in = new BufferedReader(new InputStreamReader(conn.getInputStream()))) {
                return IOUtils.toString(in);
            }
        } catch (Exception e) {
            logger.error("Exception occurred while obtaining tokens", e);
            return null;
        }
    }
}