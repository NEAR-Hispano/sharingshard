<template>
<div class="home" :key="home_refresh">
  <v-container>
    <v-row>
      <v-col md="12">
        <v-container class="my-5">
          <h1>SharingShard</h1>
          <h2>Alpha Version</h2>
          <h2 class="blue white--text">Monetize youy knowledge and experience<br>helping people to solve problems</h2>

<p class="textBoxLeft">You are involved in solving problems while creating new ventures, developing projects, or providing services. While you do your work every day, you face challenges that require getting knowledge and experience from people in the industry.</p>
 
<p class="textBoxLeft">SharingShard helps you share scenarios, situations, and problems you find on YouTube videos while looking for a solution. The goal of SharingShard is to find the help you need by sharing a video that shows the problem you want to solve. </p>
 
<p class="textBoxLeft">Commonly, specific scenarios for the problem you are looking how to find a solution are a short part of long videos. By sharing the moment you discover in the video with an open community, you can ask for specific advice, help, or a complete solution.</p>
 
<p class="textBoxLeft">People will be interested in helping you because you offer a financial reward for the best Point of View you receive, keeping the conversation open for more extensive participation in solving the challenge.</p>
 
<p class="textBoxLeft">Explore how you can get help and be an Experience Producer or play on the other side of the table as a problem solver providing Points of View.</p>

<!--

          <h3>Welcome</h3>
          <h3>Register</h3>
          <h3>Collaborate as an Experience Producer</h3>
          <h3>Collaborate with your Points of View</h3>
          <h3>Monetize your knowledge</h3>
          <h3>Be part of our community of knowledge producers</h3>
-->
        </v-container>
      </v-col>
    </v-row>
  </v-container>

  <v-container>
    <v-row>
      <v-col md="12">
<!--
      <p>User Logged: {{ user_logged }}</p>
      <p>Login status: {{login_status}}</p>
-->
        <v-container v-if=!this.login_status_b>
          <p><a :href="nearWallet">Sign</a> or <a :href="createWallet">Create an account</a></p>
        </v-container>

<!--
        <v-container  v-if=this.login_status_b>
          <h2>Logged</h2>
        </v-container>
-->
      </v-col>
    </v-row>
  </v-container>

  <v-divider class="mx-4" inset></v-divider>

  <v-container>
    <v-row md-12>
      <v-col md-6>
        <v-card
          class="mx-auto my-12"
          max-width="500"
        >
        <v-card-title>Be an experienced producer.</v-card-title>

          <v-img
                height="250"
                src="../assets/challenge.png"
          ></v-img>
        </v-card>

      </v-col>

      <v-col md-6>
        <v-card-text>
          <v-row
            align="start"
            class="mx-0"
          >
            <p class="textBoxLeft">An experience consists of finding a moment in a video by marking the scene that provides the reference to look for points of view. This scene happens in time within the video, where you have a reference to the context that helps people understand the problem to be solved.</p>
            <p class="textBoxLeft"> People will provide points of view focusing on how to solve the challenge shown at the moment.</p>
            <p class="textBoxLeft">Each moment has assigned a reward for the best Point of View allocated by the creator of the experience.</p>
          </v-row>
        </v-card-text>
      </v-col>
    </v-row>
  </v-container>
  <v-divider class="mx-4" inset></v-divider>
  <v-container>
    <v-row md-12>
      <v-col md-6>
            <v-card-text>
              <v-row
                align="start"
                class="mx-0"
              >

              <p class="textBoxLeft">Each experience is open to receiving one point of view from each community member. The Points of View are open to everybody to share ideas and provide a better understanding of the problem, enhance the current solution, and more.</p>
              <p class="textBoxLeft">After some time that the Experience is open, the person asking for help closes the experience and awards the reward to the best Point of View.</p>
              <p class="textBoxLeft">You, as a collaborator, will win earn Near currency sent to your wallet.
.</p>
              </v-row>
            </v-card-text>

      </v-col>

      <v-col md-6>
        <v-card
          class="mx-auto my-12"
          max-width="500"
        >
        <v-card-title>Collaborate with your Points of View</v-card-title>

          <v-img
                height="250"
                src="../assets/solving.png"
          ></v-img>
        </v-card>

      </v-col>


    </v-row>
  </v-container>

  <v-divider class="mx-4" inset></v-divider>


  <v-container>
    <v-row md-12>
      <v-col md-6>
        <v-card
          class="mx-auto my-12"
          max-width="500"
        >
        <v-card-title>Monetize your knowledge</v-card-title>

          <v-img
                height="250"
                src="../assets/community.png"
          ></v-img>
        </v-card>

      </v-col>

      <v-col md-6>
        <v-card-text>
          <v-row
            align="start"
            class="mx-0"
          >
            <p class="textBoxLeft">SharingShard helps people with an interest in participating in being part of solutions easily and attractively.</p>
          </v-row>
        </v-card-text>
      </v-col>
    </v-row>
  </v-container>

  <v-divider class="mx-4" inset></v-divider>

  <v-container>
    <v-row md-12>
      <v-col md-6>
            <v-card-text>
              <v-row
                align="start"
                class="mx-0"
              >

              <p class="textBoxLeft">Don\´t wait anymore. Think about a challenge you need to solve and ask for help to solve it fast and reliably in an open space to share your knowledge and experience. At the same time, you earn for your participation while learning from the community new Points of View.</p>
              </v-row>
            </v-card-text>

      </v-col>

      <v-col md-6>
        <v-card
          class="mx-auto my-12"
          max-width="500"
        >
        <v-card-title>Be part of our community</v-card-title>

          <v-img
                height="250"
                src="../assets/monetize.png"
          ></v-img>
        </v-card>

      </v-col>


    </v-row>
  </v-container>



</div>

</template>



<script>
import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores } = nearAPI;

// const CONTRACT_ID = "dev-1658426475128-11579451230587";
const config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  createWalletUrl: 'https://wallet.testnet.near.org/create',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org'
};


export default {
  name: 'HomeView',

  data() {
    return {
      user_logged: "none",
      login_status: "Logut",
      login_status_b: false,

      createWallet: "",
      nearWallet: "",

      home_refresh: 0,
    }
  },

  created(){
    this.nearWallet = config.walletUrl;
    this.createWallet = config.createWalletUrl;
    this.is_Logged();
  },

  methods: {

   forceRerender() {
      this.home_refresh += 1;
    },

    async is_Logged() {

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        if (wallet.isSignedIn()) {
          this.user_logged = wallet.getAccountId();
          this.login_status = this.user_logged;
          this.login_status_b = true;          
        } else {
          this.login_status = "Logout"; 
          this.login_status_b = false;
        }
    },
  },

  components: {
    // HelloWorld
  }
}



</script>
<style>
  .textBoxLeft {
    text-align:left;
    font-size: 24px

  }
</style>