<script lang="ts">
    import { env } from "$env/dynamic/public";
    const apiUrl = env.PUBLIC_API_URL;
</script>

<form method="POST" action={apiUrl+"/api/blog"}>
<label for="title">title</label>
<input type="text" id="title" name="title"><br />
<label for="body">body</label>
<input type="text" id="body" name="body"><br />
<label for="password">password</label>
<input type="password" id="password" name="password"><br />
<input type="submit" value="post">
</form>