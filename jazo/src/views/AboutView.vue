<template>
  <div class="about">
    <h1 class="subheading grey--text">User Registration</h1>



    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=activateUserDialog()
      >
        Create User
      </v-btn>
      
      <v-spacer></v-spacer>


      <v-dialog
        v-model="createUserDialog"
        max-width="1000px"
        >
        <template v-slot:activator="{ on, attrs }">
          <v-btn
            color="primary"
            dark
            class="mb-2 text-right"
            v-bind="attrs"
            v-on="on"
          >
            Registration
          </v-btn>
        </template>
        <v-card>
          <v-card-title class="text-h5 grey lighten-2">
            New User
          </v-card-title>

          <v-card-text>
            <v-container>
              <v-row>
                <v-text-field
                  v-model="userProfile.name"
                  label="Name"
                ></v-text-field>
              </v-row>

              <v-divider
                class="mx-4"
                inset
                vertical
              ></v-divider>

              <v-row>
                <v-text-field
                  v-model="userProfile.wallet"
                  label="wallet"
                ></v-text-field>
              </v-row>
              <v-row>
                <v-text-field
                  v-model="userProfile.discord"
                  label="Discord -- with the four digits"
                ></v-text-field>
              </v-row>

              <v-row>
                <v-text-field class="text-right"
                  v-model="userProfile.email"
                  label="E-mail"
                ></v-text-field>
              </v-row>

              <v-row>
                <v-text-field class="text-right"
                  v-model="userProfile.interests"
                  label="Areas of Interest"
                ></v-text-field>
              </v-row>

            </v-container>
          </v-card-text>

          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn
              color="blue darken-1"
              text
              @click="cancelUser()"
            >
              Cancel
            </v-btn>
            <v-btn
              color="blue darken-1"
              text
              @click="createUser()"
            >
              Create
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>

          <v-card>
            <v-responsive>
              <iframe
                  src="https://youtube.com/embed/WCUGI8PGcGw"

                  frameborder="0"
                  allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
                  allowfullscreen
              ></iframe>
              <p>otro video</p>

    <LazyYoutube
      ref="youtubeLazyVideo"
      max-width="720px"
      aspect-ratio="16:9"
      thumbnail-quality="standard"
      src="https://www.youtube.com/watch?v=TcMBFSGVi1c" 
    />

    <div class="buttons">
      <v-btn @click="handleClick('playVideo', 'youtubeLazyVideo')">
        Play
      </v-btn>
      <v-btn @click="handleClick('pauseVideo', 'youtubeLazyVideo')">
        Pause
      </v-btn>
      <v-btn @click="handleClick('stopVideo', 'youtubeLazyVideo')">
        Stop
      </v-btn>
      <v-btn @click="handleClick('resetView', 'youtubeLazyVideo')">
        Reset
      </v-btn>
    </div>

<!--  https://vuejsexamples.com/embed-a-youtube-player-easily-and-lazy-load-the-video-with-vue/  
      https://github.com/seeratawan01/vue-lazytube -->

    <LazyYoutube
      ref="youtubeLazyVideo"
      max-width="720px"
      aspect-ratio="16:9"
      thumbnail-quality="standard"
      src="https://youtu.be/WCUGI8PGcGw" 
    />

    <div class="buttons">
      <v-btn @click="handleClick('playVideo', 'youtubeLazyVideo')">
        Play
      </v-btn>
      <v-btn @click="handleClick('pauseVideo', 'youtubeLazyVideo')">
        Pause
      </v-btn>
      <v-btn @click="handleClick('stopVideo', 'youtubeLazyVideo')">
        Stop
      </v-btn>
      <v-btn @click="handleClick('resetView', 'youtubeLazyVideo')">
        Reset
      </v-btn>
    </div>



            </v-responsive>
        </v-card>


  </v-container>
</div>


</template>

<script>
  import * as nearAPI from 'near-api-js'
  const { connect, WalletConnection, keyStores, Contract } = nearAPI;

  const CONTRACT_ID = "dev-1658426475128-11579451230587";
  const config = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org'
  };

  export default {
    name: "AboutView",




    data(){
      return {
        createUserDialog: false,

        statusT: "inicial",

        userProfile: {
          name: "jorge", 
          wallet: "",
          discord: "zavala_disc", 
          email: "jorgez.info@gmail.com", 
          interests: 8,
        },

        setUserStatus: 0,
      }
    },

    methods: {

      handleClick(event, ref) {
          this.$refs[ref][event]();
      },

      activateUserDialog(){
        this.createUserDialog = true;
      },

      cancelUser(){
        this.createUserDialog = false;
      },

      async createUser(){

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['set_user'],
          sender: wallet.account(),
        });


        this.setUserStatus = await contract.set_user({
          name: this.userProfile.name, 
          wallet: wallet.getAccountId(),
          discord: this.userProfile.discord, 
          email: this.userProfile.email, 
          interests: 8,
        });


        this.statusT = "set user: " + this.setUserStatus;
        console.log( this.setUserStatus );


        this.createUserDialog = false;

      },
    },
  }
</script>

<style>
.buttons00 {
  margin-top: 25px;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: 100%;

}
</style>