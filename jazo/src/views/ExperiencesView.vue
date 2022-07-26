<template>
<div class="c_experienciesview">
    <h1 class="subheading grey--text">Experiences</h1>
    <p>Choose the experiences where you want to collaborate</p>
    <p>Get ready to participate providing your points of views or asking for help developing your own experiences</p>
    
    <v-container fluid class="my-5">
      <v-toolbar-title class="text-right">
        <v-container fluid>
      
      <v-row md-12 >
      <v-col md-3>You are: {{ users_value }}</v-col>
      <v-col md-3> Experience List: {{ exp_list }} -- exp_list: {{ exp_list.length }} -- {{experienciesview.length}}  </v-col>
      <v-col md-3> near_env: {{ near_env }} </v-col>
      <v-col md-3> {{select_user_f}} </v-col>
      </v-row>

          <v-row>
          <v-col md-2>
            <v-select label="Users" :items=users @input="select_user"></v-select>

          </v-col>
          <v-col md-8>
            <p> </p>
          </v-col>
          <v-col md-2>

            <!--------------------- New Experience Dialog ------------------------->

            <v-dialog
              v-model="new_experience"
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
                  New Experience
                </v-btn>
              </template>
              <v-card>
                <v-card-title class="text-h5 grey lighten-2">
                  New Experience
                </v-card-title>

                <v-card-text>
                  <v-container>
                    <v-row>
                        <v-text-field
                          v-model="newExp.title"
                          label="Titulo"
                        ></v-text-field>
                      </v-row>
                      <v-row>
                        <v-textarea
                          rows="5"
                          row-height="5"
                          v-model="newExp.description"
                          label="Description"
                        ></v-textarea>
                      </v-row>
                    <v-row>
                        <v-text-field
                          v-model="newExp.url"
                          label="URL"
                        ></v-text-field>
                    </v-row>

                    <v-row>
                      <v-col>
                        <v-text-field class="text-right"
                          v-model="newExp.rewards"
                          label="Rewards"
                        ></v-text-field>
                      </v-col>

                      <v-divider
                        class="mx-4"
                        inset
                        vertical
                      ></v-divider>

                      <v-col>
                        <v-text-field class="text-right"
                          v-model="newExp.time"
                          label="Time to watch the moment in the video"
                        ></v-text-field>
                      </v-col>
                    </v-row>


                  </v-container>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn
                    color="blue darken-1"
                    text
                    @click="closeExp"
                  >
                    Cancel
                  </v-btn>
                  <v-btn
                    color="blue darken-1"
                    text
                    @click="saveExp"
                  >
                    Save
                  </v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>

            <!-- Activate experience -->

            <v-dialog
              v-model="dialogActivation"
              max-width="1000px"
              >

              <v-card>
                <v-card-title class="text-h5 grey lighten-2">
                    Activate the Experience and Deposit the Money for the Reward
                </v-card-title>

                <v-card-text>
                    <v-row>
                      <v-col>
                        <v-text-field class="text-right"
                          v-model="newExpActivation.deposit"
                          label="Deposit"
                          suffix="NEARS"
                        ></v-text-field>
                      </v-col>
                      <v-col>
                        <v-text-field class="text-right"
                          v-model=feex
                          label="Activation Fee"
                          suffix="NEARS"
                          readonly
                        ></v-text-field>
                      </v-col>
                      <v-col>
                        <v-text-field class="text-right"
                          v-model=rewardx
                          label="Reward"
                          suffix="NEARS"
                          readonly
                        ></v-text-field>
                      </v-col>
                    </v-row>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn
                    color="blue darken-1"
                    text
                    @click="closeExpActivation"
                  >
                    Cancel
                  </v-btn>
                  <v-btn
                    color="blue darken-1"
                    text
                    @click="saveExpActivitation"
                  >
                    Activate
                  </v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>


          </v-col>
          </v-row>
        </v-container>
      </v-toolbar-title>

      <h2 id=top_card v-if="experienciesview.length==0">Sorry, we do not have any experiences availabe at this time</h2>
      <v-row id=top_card :key="experiences_cards">
      <!--
        <v-col xs12 sm6 md4 lg3 v-for="item in experienciesview.length" :key="item">
      -->
        <v-col cols="12" md="3" v-for="item in experienciesview.length" :key="item">
          <v-card>

        <LazyYoutube
          ref="youtubeLazyVideo"
          max-width="720px"
          aspect-ratio="16:9"
          thumbnail-quality="standard"
          :src=experienciesview[item-1].url
        />


          <!--
            <v-responsive>
              <iframe
                  width="100%"
                  height="100%"
                  :src=experienciesview[item-1].url
                  frameborder="0"
                  allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
                  allowfullscreen
              ></iframe>
            </v-responsive>
          -->
          <v-card-text>
            <div v-if=!hide_it class="subheading text-sm-left">Video: {{ experienciesview[item-1].video_n }}</div>
            <div class="subheading text-sm-left">{{ experienciesview[item-1].title }}</div>

            <div class="subheading text-sm-left">{{ experienciesview[item-1].description }}</div>
            <div class="grey--text"><p class="text-sm-left">Reward: {{ experienciesview[item-1].reward }}</p></div>
            <div class="subheading text-sm-left">By: {{ experienciesview[item-1].owner }}</div>

            <v-divider></v-divider>

            <div class="grey--text">{{ experienciesview[item-1].url }}</div>
          </v-card-text>

          <v-card-actions v-show="experienciesview[item-1].status=='InProcess'&&experienciesview[item-1].owner==owner" >
            <v-btn text color="grey" @click="goToMoment( experienciesview[item-1].video_n )" >
              <v-icon small left>mdi-message</v-icon>
              <span class="red white--text"> Add a Moment </span>
            </v-btn>
          </v-card-actions>

          <v-card-actions>
            <v-btn text color="grey" @click="goToPoV( experienciesview[item-1].video_n )">
              <v-icon small left>mdi-message</v-icon>
              <span class="">PoV</span>
            </v-btn>
          </v-card-actions>

          <v-card-actions>
            <div v-if="experienciesview[item-1].status=='Active'" class="green white--text mx-auto elevation-6 rounded-lg" ><b>Active</b></div>
            <div v-if="experienciesview[item-1].status=='InProcess'" class="black white--text mx-auto elevation-6 rounded-lg" ><b>InProcess</b></div>
            <div v-if="experienciesview[item-1].status=='Closed'" class="red white--text mx-auto elevation-6 rounded-lg" ><b>Closed</b></div>

            <v-btn v-show="experienciesview[item-1].status=='InProcess'&&experienciesview[item-1].owner==owner" text color="grey" @click="goToAction( experienciesview[item-1].video_n, 'InProcess' )">
              <span class="">Activate</span>
            </v-btn>


          </v-card-actions>

        </v-card>
      </v-col>

    </v-row>
  </v-container>
</div>
</template>

<script>
import * as nearAPI from 'near-api-js'
import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)
const store = new Vuex.Store({
  state: {
    select_user: "",
  },
});

store.state.select_user = "";
//const xxx = store.state.select_user;
//store.state.select_user = xxx;


const { connect, WalletConnection, keyStores, Contract, utils } = nearAPI;

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
  name: "ExperiencesView",
  /* https://learnvue.co/tutorials/vue-lifecycle-hooks-guide
    created()
    mounted()
    updated()
  */
  created(){
    // alert( "Created: select user init, user: " + this.users_value + "Visit_time: " + this.visit_num);
    this.disp_experiences( "-1" ); // display de experiences of the current user
    this.visit_num +=1;

  },  

/*
struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    topic: u8,
    reward: Balance,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: UnorderedMap<AccountId, String>,
    status: String,
}

"wallet": "zavala55.testnet",

"experience_name": "Aprendizajes en el Silicon Valley", 
"description": "Narración de la Experiencia de Virgilio Raiden en su visita a SV", 
"url": "https://youtube.com/embed/WCUGI8PGcGw",
"reward": 10, 
"expire_date":20, 
"topic":8
*/

  data() {
    return {
      number_of_experiences: 0,
      exp_list: [],
      exp_info: "",
      experienciesview: [],

      new_experience: false,
      newExp: {
        title: "",
        description: "",
        owner: "",
        url: "",
        rewards: 0,
        moment: "",
        time: 0,
        expire_date: 0,
        topic: 0,

      },
      hide_it: true,

      users: [],
      users_value: "",

      near_env: `${process.env.VUE_APP_NEAR_ENV}`,

      newExpActivation: {
          video_n: 0,
          deposit: 0,
          fee: 0,
          total: 0,
      },

      dialogActivation: false,
      feeCharge: 0,

      visit_num: 0,

      experiences_cards: 0,
      owner: "",
    }
  },

  computed: {

      feex: function() {
         return parseFloat(this.newExpActivation.deposit * this.feeCharge ).toFixed(2);
      },

      rewardx: function() {
        return (parseFloat(this.newExpActivation.deposit) - parseFloat(this.newExpActivation.deposit * this.feeCharge )).toFixed(2);
      },

      select_user_f(){
        return ( store.state.select_user);
      }
    },

  methods: {

    forceRerender() {
      this.experiences_cards += 1;
    },

    goToAction( exp_num, action){
      this.newExpActivation.video_n = exp_num;
      exp_num = action;
      action = exp_num;
      this.dialogActivation = true;
    },

    async saveExpActivitation(){

      const near = await connect(config);
      const wallet = new WalletConnection(near, 'SharingShard');

      const contract = new Contract( wallet.account(), CONTRACT_ID, 
      { 
        changeMethods:  ['activate_experience'],
        viewMethods: ['get_experience'],
        sender: wallet.account(),
      });

      this.exp_info = await contract.get_experience({
        video_n: this.newExpActivation.video_n // video_id
      });

//const amountInYocto = utils.format.parseNearAmount("10");

      this.expInfo = await contract.activate_experience(
        { 
          video_n : this.newExpActivation.video_n,
        }, 
        300000000000000,
        utils.format.parseNearAmount(this.newExpActivation.deposit.toString()) )
      .catch((err) => {
        alert ( err );
        return err;
      });
      this.dialogActivation = false;
    },


    closeExpActivation(){
      this.dialogActivation = false;
    },

    select_user(item) {

      // alert( "INIT: select user init, user: " + this.users_value );

      const parent = document.querySelector('#top_card');
      while (parent.firstChild) {
        parent.removeChild(parent.firstChild);
      }

      this.users_value = item;
      this.disp_experiences(this.users_value);
      // alert( "END: select user init, user: " + this.users_value + "Visit_time: " + this.visit_num);
      this.visit_num +=1;
      store.state.select_user = this.users_value;

    },


    /*
    * Display de experiences of the owner 
    * -1 -- means current signed user
    */
    async disp_experiences( exp_owner ){
      const near = await connect(config);
      const wallet = new WalletConnection(near, 'SharingShard');

      if (wallet.isSignedIn()) {
        console.log("ExperienceView -- Signed");

        if ( store.state.select_user != "" )
          this.newExp.owner = store.state.select_user;
        else {
          if ( exp_owner == -1 ) {
            this.newExp.owner = wallet.getAccountId();
            this.owner=wallet.getAccountId();
          } else {
            this.newExp.owner = exp_owner;
          }
        }


        console.log( "These are the experiences of the user: " + this.newExp.owner )

        const contract = new Contract(wallet.account(), CONTRACT_ID, {
          viewMethods:  ['get_number_of_experiences', 'get_user_exp','get_experience', 'get_list_of_users', 'get_fee'],
          sender: wallet.account()
        });

/*
 *  populate the list of all available users
 */
        this.users = await contract.get_list_of_users();
        this.users_value = wallet.getAccountId();

/*
 *  set the fee rate from the blockchain
 */
        this.feeCharge = await contract.get_fee() / 100;

        // use a contract view method
        this.number_of_experiences = await contract.get_number_of_experiences();
        console.log( "get_number_of_experiences" + this.number_of_experiences );

        this.exp_list = await contract.get_user_exp({
          "wallet": this.newExp.owner
        });
        console.log( this.exp_list );
        this.experienciesview = [];

        let j = 0;

        for ( let i = 0 ; i < this.exp_list.length ; i++  ){
          console.log( "i: " + i + " length: " +this.exp_list.length + " video_n: " + this.exp_list[i]);

          this.exp_info = await contract.get_experience({
            video_n: this.exp_list[i]
          });
          console.log( this.exp_info );

/*
 *  here we validate if the experiences are from the user and allow to closed or activate 
 *  Experiences from other users are only show the ones that are active
 * 
 *  Display my own experiences
 */

          if (this.exp_info.owner == wallet.getAccountId()) {
            this.experienciesview[j] = this.exp_info;
            this.experienciesview[j++].video_n = this.exp_list[i];
          } else {
            if ( this.exp_info.status == "Active" ) {
              this.experienciesview[j] = this.exp_info;
              this.experienciesview[j++].video_n = this.exp_list[i];
            }
          }


/*
 *  Display the active experiences from other users
 */

        }
        console.log( this.experienciesview );
      } else {
        //<p> please signin as an active user </p>
                console.log("ExperienceView -- NOT SIGNED");
      }
      this.forceRerender();
    },


    goToMoment( video_id ){
      // in moment we use QUERY as a reference to pass information between the source page and the moment page
      // this.$router.push('/moment');
    // console.log("goToMoment\n");
      //if the route accepts params, you can also use
      this.$router.push({
        name:'moment', 
        params: {video_id: video_id  }, 
        } );
    },
    goToPoV( video_id ){

      console.log( "video:id: " + video_id);

      // in moment we use param as a reference
      // in PoV we use PARAMETER as a reference to pass information between the source page and the moment page

    // this.$router.push('/moment');
    // console.log("goToPoV\n");
      //if the route accepts params, you can also use
      this.$router.push({
        name:'pov', 
        params: {video_id: video_id  }, 
        } );
    },

    closeExp () {
      this.new_experience = false;
    },


    async saveExp(){
      this.new_experience = false;

      console.log( "Save the Experience");

      const near = await connect(config);
      const wallet = new WalletConnection(near, 'SharingShard');

      const contract = new Contract( wallet.account(), CONTRACT_ID, 
      { 
        changeMethods:  ['set_experience'],
        sender: wallet.account(),
      });

      this.expInfo = await contract.set_experience(
        { 
        experience_name: this.newExp.title, 
        description: this.newExp.description,
        url: this.newExp.url,
        reward: parseInt(this.newExp.rewards),
        moment: this.newExp.moment,
        time: parseInt(this.newExp.time),
        expire_date: this.newExp.expire_date,
        topic: this.newExp.topic,
        }, 
        300000000000000,
        0
      );
      this.disp_experiences( -1 );
      console.log( this.expInfo );
    },



  },  
}
</script>
