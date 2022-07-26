<template>
  <v-toolbar text>
      <v-toolbar-title class="text-uppercase grey--text">
        <v-btn router-link to="/">
          <span class="font-weight-light">Sharing</span>
          <span class="font-weight-black">Shard</span>
        </v-btn>
      </v-toolbar-title>

      <v-spacer></v-spacer>

      <span v-for="command in commands" :key="command.text" >
            <v-btn router :to="command.route" text :color="command.color" >{{ command.text }}</v-btn>
      </span>

      <v-btn text color="grey" @click=login_action>
        <span>{{ this.login_status }}</span>
        <v-icon>mdi-logout-variant</v-icon>
      </v-btn>

      <v-dialog
        v-model="createUserDialog"
        max-width="1000px"
        >
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

  </v-toolbar>

</template>

<script>
import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores, Contract} = nearAPI;

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
    name: 'NavBar',
  data() {
    return {
      drawer: false,
      login_status: "Logout",
      user_logged: "",
      commands: [
        { color: "green", text: 'Experiences', route: '/experiences'},
        { color: "blue", text: 'Moment', route: '/moment'},
        { color: "red", text: 'Points of View', route: '/pov'},
        { color: "orange", text: 'Statistics', route: '/stats'},        
        { color: "brown", text: 'About', route: '/about'},
      ],

      createUserDialog: false,

      userProfile: {
        name: "", 
        wallet: "",
        discord: "", 
        email: "", 
        interests: 8,
      },

      setUserStatus: 0,
    }
  },
  created(){
    this.is_Logged();
  },

  methods: {
    async login_action() {

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        if (this.login_status == "Login") {
          await wallet.requestSignIn( CONTRACT_ID, 'SharingShard');
          this.user_logged = wallet.getAccountId();
          this.login_status = this.user_logged;  //"Logout"
          // console.log( "Logout");

          
// this is needed?
          const contract = new Contract( wallet.account(), CONTRACT_ID, 
          { 
            viewMethods: ['user_exist'],
            sender: wallet.account(),
          });

          const userExist = contract.user_exist({
            wallet: this.user_logged,
          })

          if ( !userExist )
            this.createUserDialog = true;
// who knows?



        } else {
          wallet.signOut();
          this.login_status = "Login";
          // console.log( "Login " ); 
          this.goToHome();
        }
    },
    async is_Logged() {
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        if (wallet.isSignedIn()) {
          this.user_logged = wallet.getAccountId();
          this.login_status = this.user_logged;

          const contract = new Contract( wallet.account(), CONTRACT_ID, 
          { 
            viewMethods: ['user_exist'],
            sender: wallet.account(),
          });

          const userExist = await contract.user_exist({
            wallet: this.user_logged,
          })

          if ( !userExist ){
            this.userProfile.wallet = wallet.getAccountId();
            this.createUserDialog = true;
          }
          this.goToExperiences();

        } else {
          this.login_status = "Login";
          this.goToHome();
        }
    },
    goToHome(){
      if (this.$route.path !== "/") {
        this.$router.push("/");
      }
    },

    goToExperiences(){
      this.$router.push({ name:'experiencesview' }); 
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

      const contract = new Contract( wallet.account(), CONTRACT_ID, { 
        changeMethods:  ['set_user'],
        sender: wallet.account(),
      });


      this.setUserStatus = await contract.set_user({
        wallet: "zavala55.testnet", // wallet.getAccountId(),
        name: this.userProfile.name, 
        discord: this.userProfile.discord, 
        email: this.userProfile.email, 
        interests: 8,
      }
      );


      this.statusT = "set user: " + this.setUserStatus;
      console.log( this.setUserStatus );


      this.createUserDialog = false;

    },  

  }
}
</script>

<style>

</style>
