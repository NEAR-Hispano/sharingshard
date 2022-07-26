<template>
<div class="momentview">
<v-main fluid>
    <v-layout row wrap>
      <v-flex xs12 class="text-xs-center" mt-5>
        <h1 class="subheading grey--text">Moment</h1>
      </v-flex>
      <v-flex xs12 class="text-xs-center" mt-3>
        <p>Define a moment in a video that helps to understand a challenge</p>
      </v-flex>
    </v-layout>

      <v-row>
        Discover the value in this learning experience: {{exp_info.title}}
        <p>{{exp_info}}</p>
      </v-row>

      <v-row  style="height: 500px;">
        <v-col md="6">
{{urlWithTime}}
    <LazyYoutube
      ref="youtubeLazyVideo"
      max-width="720px"
      aspect-ratio="16:9"
      thumbnail-quality="standard"
      :src=urlWithTime
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


<!--
          <iframe
              width="100%"
              height="100%"
              :src=exp_info.url
              frameborder="0"
              allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
              allowfullscreen
          ></iframe>
-->          
        </v-col>

        <v-col md="6">
        <v-container id="input-usage" >
          <v-row>
            Time: 
            <v-text-field  suffix="seconds" v-model="exp_info.time">
            Label="Introduce the timemark where you define the moment in the video"
            </v-text-field>
          </v-row>
        </v-container>
          <v-row>
            <v-col
                cols="40"
                md="12"
              >
              <v-textarea
                filled
                v-model="exp_info.moment"

                name="input-7-4"
                label="Moment"
                value=exp_info.moment
                auto-grow
                rows="15"
                row-height="15"
              ></v-textarea>

            </v-col>
          </v-row>
        </v-col>
      </v-row>
    
      <v-row>
        <v-col md-6 >
          <p>URL del video:</p><span>{{this.urlWithTime}}</span>


        </v-col>


          <v-col md-6 yellow filled dark show-shatches=true color="black">
            <v-row dark elevation=10 outlined>
              <v-col md-4> 
                <v-btn 
                  depressed
                  color="primary"
                  @click=clickea_edit_moment()
                  v-show=future_feature
                >
                  Edit
                </v-btn>
              </v-col>

              <v-col md-4>
                <v-btn
                  depressed
                  color="primary"
                  @click=clickea_insert_moment
                  v-show=edit_by_owner
                >
                  Insert
                </v-btn>
              </v-col> 

              <v-col md-4> 
                <v-btn 
                  depressed
                  color="primary"
                  @click=clickea_delete_moment()
                  Dark=true
                  outlined
                  v-show=future_feature
                >
                  Delete
                </v-btn>
              </v-col> 
            </v-row>


          </v-col>
        
      </v-row>
    </v-main>

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
    name: "MomentView",
    created(){
      this.exp_id = this.$route.params.video_id
      this.disp_moment(this.exp_id );
    },
    updated(){
        console.log("updated: " + window.location.href);
    },

    computed: {
      urlWithTime: function(){
        return this.exp_info.url + "?t=" + this.exp_info.time;
      }
    },


    data(){
      return {
        video_info: 0,
        exp_info: "",

        edit_by_owner: false,
        future_feature: false,
      }
    },

    methods: {
      handleClick(event, ref) {
          this.$refs[ref][event]();
      },


      async disp_moment(video_id){
        
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract(wallet.account(), CONTRACT_ID, {
          viewMethods:  ['get_experience'],
          sender: wallet.account()
        });

        this.exp_info = await contract.get_experience({
          video_n: video_id
        });




        if ( this.exp_info.owner == wallet.getAccountId() && this.exp_info.status == "InProcess" )
          this.edit_by_owner = true;

        console.log( this.exp_info );
        
      },
      async clickea_insert_moment(){

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, { 
          changeMethods:  ['set_moment_comment'],
          sender: wallet.account(),
        });

        this.expInfo = await contract.set_moment_comment(
          { 
          video_n: this.exp_id,
          comment: this.exp_info.moment
          }, 
          300000000000000,
          0
        );

        this.statusT = "set_moment_comment result: " + this.expInfo;
        console.log( this.expInfo );

        console.log( "clickea_insert_moment");

        this.$router.push({
          name:'experiencesview', 
        });

      },
      clickea_edit_moment(){
        console.log( "clickea_edit_moment");
      },
      clickea_delete_moment(){
        console.log( "clickea_delete_moment");
      },

    },
  }
</script>
